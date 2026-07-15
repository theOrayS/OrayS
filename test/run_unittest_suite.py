#!/usr/bin/env python3
"""Execute one canonical unittest module with exact runtime identity binding."""

from __future__ import annotations

import argparse
import ast
import functools
import hashlib
import importlib.util
import inspect
import os
import sys
import tempfile
import traceback
import types
import unittest
from collections import Counter
from pathlib import Path
from typing import Any, Callable

sys.dont_write_bytecode = True
sys.pycache_prefix = "/dev/null"
os.environ["PYTHONDONTWRITEBYTECODE"] = "1"
os.environ["PYTHONPYCACHEPREFIX"] = "/dev/null"
TEST_ROOT = Path(__file__).resolve().parent
if str(TEST_ROOT) not in sys.path:
    sys.path.insert(0, str(TEST_ROOT))

import run_suite as suite_contract


class HarnessIntegrityError(RuntimeError):
    """The declared unittest identities could not be bound to real execution."""


class FileDescriptorTee:
    """Capture and relay both descriptors even if the harness dies by signal."""

    def __init__(self) -> None:
        self.captured: dict[int, bytearray] = {1: bytearray(), 2: bytearray()}
        self._saved: dict[int, int] = {}
        self._readers: dict[int, int] = {}
        self._writers: dict[int, int] = {}
        self._relay_pids: dict[int, int] = {}
        self._capture_files: dict[int, Any] = {}

    @staticmethod
    def _flush_python_streams() -> None:
        for stream in (sys.stdout, sys.stderr):
            try:
                stream.flush()
            except (AttributeError, OSError):
                pass

    @staticmethod
    def _write_all(target_fd: int, chunk: bytes) -> None:
        view = memoryview(chunk)
        while view:
            written = os.write(target_fd, view)
            view = view[written:]

    def _relay_child(self, target_fd: int) -> None:
        read_fd = self._readers[target_fd]
        saved_fd = self._saved[target_fd]
        capture_fd = self._capture_files[target_fd].fileno()
        retained = {read_fd, saved_fd, capture_fd}
        inherited = {
            1,
            2,
            *self._readers.values(),
            *self._writers.values(),
            *self._saved.values(),
            *(capture.fileno() for capture in self._capture_files.values()),
        }
        for descriptor in inherited - retained:
            try:
                os.close(descriptor)
            except OSError:
                pass
        try:
            while True:
                chunk = os.read(read_fd, 65536)
                if not chunk:
                    break
                self._write_all(capture_fd, chunk)
                self._write_all(saved_fd, chunk)
        except BaseException:
            os._exit(125)
        finally:
            for descriptor in retained:
                try:
                    os.close(descriptor)
                except OSError:
                    pass
        os._exit(0)

    def __enter__(self) -> "FileDescriptorTee":
        self._flush_python_streams()
        for target_fd in (1, 2):
            read_fd, write_fd = os.pipe()
            self._saved[target_fd] = os.dup(target_fd)
            self._readers[target_fd] = read_fd
            self._writers[target_fd] = write_fd
            self._capture_files[target_fd] = tempfile.TemporaryFile(mode="w+b")
        try:
            for target_fd in (1, 2):
                pid = os.fork()
                if pid == 0:
                    self._relay_child(target_fd)
                self._relay_pids[target_fd] = pid
            for target_fd in (1, 2):
                os.dup2(self._writers[target_fd], target_fd)
                os.close(self._writers[target_fd])
                os.close(self._readers[target_fd])
        except BaseException:
            for descriptor in (*self._writers.values(), *self._readers.values()):
                try:
                    os.close(descriptor)
                except OSError:
                    pass
            for saved_fd in self._saved.values():
                try:
                    os.close(saved_fd)
                except OSError:
                    pass
            for capture in self._capture_files.values():
                capture.close()
            raise
        return self

    def __exit__(self, _exc_type: object, _exc: object, _tb: object) -> None:
        self._flush_python_streams()
        for target_fd in (1, 2):
            os.dup2(self._saved[target_fd], target_fd)
        for saved_fd in self._saved.values():
            os.close(saved_fd)
        relay_failures = []
        for target_fd in (1, 2):
            _pid, status = os.waitpid(self._relay_pids[target_fd], 0)
            if not os.WIFEXITED(status) or os.WEXITSTATUS(status) != 0:
                relay_failures.append(target_fd)
            capture = self._capture_files[target_fd]
            capture.seek(0)
            self.captured[target_fd].extend(capture.read())
            capture.close()
        if relay_failures:
            raise HarnessIntegrityError(
                f"output relay failed for file descriptors {relay_failures}"
            )

    def stdout_bytes(self) -> bytes:
        return bytes(self.captured[1])

    def stderr_bytes(self) -> bytes:
        return bytes(self.captured[2])


class BindingTextResult(unittest.TextTestResult):
    """Record the exact runtime identities started and stopped by unittest."""

    def __init__(self, *args: Any, **kwargs: Any) -> None:
        super().__init__(*args, **kwargs)
        self.started_ids: list[str] = []
        self.stopped_ids: list[str] = []
        self.outcome_events: list[tuple[str, str]] = []

    def startTest(self, test: unittest.case.TestCase) -> None:
        self.started_ids.append(test.id())
        super().startTest(test)

    def stopTest(self, test: unittest.case.TestCase) -> None:
        self.stopped_ids.append(test.id())
        super().stopTest(test)

    def addError(self, test: unittest.case.TestCase, err: tuple[type[BaseException], BaseException, types.TracebackType]) -> None:
        self.outcome_events.append(("error", test.id()))
        super().addError(test, err)

    def addFailure(self, test: unittest.case.TestCase, err: tuple[type[BaseException], BaseException, types.TracebackType]) -> None:
        self.outcome_events.append(("failure", test.id()))
        super().addFailure(test, err)

    def addSkip(self, test: unittest.case.TestCase, reason: str) -> None:
        self.outcome_events.append(("skip", test.id()))
        super().addSkip(test, reason)

    def addExpectedFailure(self, test: unittest.case.TestCase, err: tuple[type[BaseException], BaseException, types.TracebackType]) -> None:
        self.outcome_events.append(("expected-failure", test.id()))
        super().addExpectedFailure(test, err)

    def addUnexpectedSuccess(self, test: unittest.case.TestCase) -> None:
        self.outcome_events.append(("unexpected-success", test.id()))
        super().addUnexpectedSuccess(test)

    def addSubTest(
        self,
        test: unittest.case.TestCase,
        subtest: unittest.case.TestCase,
        err: tuple[type[BaseException], BaseException, types.TracebackType] | None,
    ) -> None:
        if err is not None:
            self.outcome_events.append(("subtest-error", subtest.id()))
        super().addSubTest(test, subtest, err)


TRUSTED_TEST_CASE = unittest.TestCase
TRUSTED_TEST_SUITE = unittest.TestSuite
TRUSTED_TEXT_RUNNER = unittest.TextTestRunner
TRUSTED_TEST_LOADER = unittest.TestLoader
TRUSTED_FRAMEWORK_CLASSES = (
    unittest.TestCase,
    unittest.TestSuite,
    unittest.TestResult,
    unittest.TextTestResult,
    unittest.TextTestRunner,
    unittest.TestLoader,
)
TRUSTED_FRAMEWORK_STATE = {
    framework_class: dict(framework_class.__dict__)
    for framework_class in TRUSTED_FRAMEWORK_CLASSES
}
TRUSTED_UNITTEST_BINDINGS = {
    name: getattr(unittest, name)
    for name in (
        "TestCase",
        "TestSuite",
        "TestResult",
        "TextTestResult",
        "TextTestRunner",
        "TestLoader",
        "SkipTest",
        "addModuleCleanup",
    )
}


def strict_source(path: Path) -> tuple[str, ast.Module]:
    try:
        source = path.read_text(encoding="utf-8", errors="strict")
    except UnicodeDecodeError as error:
        raise HarnessIntegrityError(
            f"{path}: invalid UTF-8 at byte offset {error.start}"
        ) from error
    except OSError as error:
        raise HarnessIntegrityError(f"cannot read {path}: {error}") from error
    try:
        return source, ast.parse(source, filename=str(path))
    except SyntaxError as error:
        raise HarnessIntegrityError(f"cannot parse {path}: {error}") from error


def verify_framework_unchanged() -> None:
    for name, expected in TRUSTED_UNITTEST_BINDINGS.items():
        if getattr(unittest, name, None) is not expected:
            raise HarnessIntegrityError(f"test module mutated unittest.{name}")
    for framework_class, expected in TRUSTED_FRAMEWORK_STATE.items():
        observed = dict(framework_class.__dict__)
        if set(observed) != set(expected) or any(
            observed[name] is not value for name, value in expected.items()
        ):
            raise HarnessIntegrityError(
                f"test module mutated unittest framework class {framework_class.__name__}"
            )


def load_module(
    path: Path,
    source: str,
) -> tuple[types.ModuleType, str, str]:
    module_name = "_orays_bound_unittest_" + hashlib.sha256(
        str(path).encode("utf-8")
    ).hexdigest()
    spec = importlib.util.spec_from_file_location(module_name, path)
    if spec is None or spec.loader is None:
        raise HarnessIntegrityError(f"cannot create an import specification for {path}")
    module = importlib.util.module_from_spec(spec)
    sys.modules[module_name] = module
    try:
        code = compile(source, str(path), "exec", dont_inherit=True, optimize=0)
        with FileDescriptorTee() as captured:
            exec(code, module.__dict__)
        captured_stdout = captured.stdout_bytes()
        captured_stderr = captured.stderr_bytes()
    except BaseException as error:
        sys.modules.pop(module_name, None)
        if isinstance(error, (SystemExit, KeyboardInterrupt)):
            raise
        traceback.print_exc(file=sys.stderr)
        raise HarnessIntegrityError(
            f"test module import raised {type(error).__name__}: {error}"
        ) from error
    if captured_stdout or captured_stderr:
        sys.modules.pop(module_name, None)
        raise HarnessIntegrityError(
            "test module emitted output during import; canonical discovery must be side-effect free"
        )
    try:
        verify_framework_unchanged()
    except BaseException:
        sys.modules.pop(module_name, None)
        raise
    return module, module_name, ""


def runtime_classes(
    module: types.ModuleType,
    module_name: str,
    path: Path,
    inventory: list[tuple[str, str, int]],
) -> dict[str, type[unittest.TestCase]]:
    expected_by_class: dict[str, list[tuple[str, int]]] = {}
    for class_name, method_name, line_number in inventory:
        expected_by_class.setdefault(class_name, []).append((method_name, line_number))

    classes: dict[str, type[unittest.TestCase]] = {}
    for class_name, expected_methods in expected_by_class.items():
        candidate = module.__dict__.get(class_name)
        if (
            not isinstance(candidate, type)
            or type(candidate) is not type
            or candidate.__bases__ != (TRUSTED_TEST_CASE,)
            or candidate.__module__ != module_name
            or candidate.__qualname__ != class_name
        ):
            raise HarnessIntegrityError(
                f"runtime class {class_name} is not the exact direct unittest.TestCase declared by {path}"
            )
        forbidden_specials = sorted(
            name
            for name in candidate.__dict__
            if name.startswith("__")
            and name not in {"__annotations__", "__doc__", "__module__"}
        )
        inherited_overrides = sorted(
            name
            for name in candidate.__dict__
            if name in TRUSTED_TEST_CASE.__dict__
            and name
            not in {
                "__doc__",
                "__module__",
                "setUp",
                "tearDown",
                "setUpClass",
                "tearDownClass",
            }
            and not name.startswith("test")
        )
        if forbidden_specials or inherited_overrides:
            raise HarnessIntegrityError(
                f"runtime class {class_name} overrides unittest execution machinery; "
                f"special={forbidden_specials}, inherited={inherited_overrides}"
            )

        expected_names = [method_name for method_name, _line in expected_methods]
        observed_names = [
            name for name in candidate.__dict__ if name.startswith("test")
        ]
        if observed_names != expected_names:
            raise HarnessIntegrityError(
                f"runtime test identity mismatch for {class_name}: "
                f"expected={expected_names}, observed={observed_names}"
            )
        for method_name, line_number in expected_methods:
            function = candidate.__dict__.get(method_name)
            if (
                not isinstance(function, types.FunctionType)
                or function.__name__ != method_name
                or function.__qualname__ != f"{class_name}.{method_name}"
                or Path(function.__code__.co_filename).resolve() != path
                or function.__code__.co_firstlineno != line_number
                or getattr(function, "__unittest_skip__", False)
                or getattr(function, "__unittest_expecting_failure__", False)
            ):
                raise HarnessIntegrityError(
                    f"runtime method {class_name}.{method_name} is not bound to its declared source identity"
                )
        classes[class_name] = candidate
    return classes


class ExecutionAudit:
    """Append-only evidence that bodies, hooks, and cleanups completed normally."""

    def __init__(self) -> None:
        self.expected: Counter[str] = Counter()
        self.entered: Counter[str] = Counter()
        self.completed: Counter[str] = Counter()
        self.raised: list[tuple[str, str]] = []
        self.invalid_returns: list[tuple[str, str]] = []
        self._cleanup_serial = 0

    def expect(self, identity: str, count: int = 1) -> None:
        self.expected[identity] += count

    def cleanup_identity(self, scope: str, function: object) -> str:
        self._cleanup_serial += 1
        name = getattr(function, "__qualname__", getattr(function, "__name__", type(function).__name__))
        identity = f"cleanup:{scope}:{self._cleanup_serial}:{name}"
        self.expect(identity)
        return identity

    def complete(self) -> bool:
        return (
            not self.raised
            and not self.invalid_returns
            and self.entered == self.expected
            and self.completed == self.expected
        )


def _close_lazy_return(value: object) -> None:
    close = getattr(value, "close", None)
    if callable(close):
        try:
            close()
        except BaseException:
            pass


def invoke_checked(
    audit: ExecutionAudit,
    identity: str,
    function: object,
    *args: object,
    post_call: object | None = None,
    **kwargs: object,
) -> None:
    if not callable(function):
        raise HarnessIntegrityError(f"{identity} is not callable")
    audit.entered[identity] += 1
    try:
        returned = function(*args, **kwargs)
    except BaseException as error:
        audit.raised.append((identity, type(error).__name__))
        raise
    finally:
        if callable(post_call):
            post_call()
    if (
        inspect.isawaitable(returned)
        or inspect.isgenerator(returned)
        or inspect.isasyncgen(returned)
        or returned is not None
    ):
        kind = type(returned).__name__
        audit.invalid_returns.append((identity, kind))
        _close_lazy_return(returned)
        raise AssertionError(
            f"{identity} returned {kind} instead of completing synchronously with None"
        )
    audit.completed[identity] += 1


def audited_cleanup(
    audit: ExecutionAudit,
    scope: str,
    function: object,
    post_call: object | None = None,
) -> object:
    identity = audit.cleanup_identity(scope, function)

    def wrapper(*args: object, **kwargs: object) -> None:
        invoke_checked(
            audit,
            identity,
            function,
            *args,
            post_call=post_call,
            **kwargs,
        )

    if isinstance(function, (types.FunctionType, types.MethodType)):
        functools.update_wrapper(wrapper, function)
    setattr(wrapper, "__orays_audited_cleanup__", True)
    return wrapper


def audit_cleanup_stack(
    audit: ExecutionAudit,
    scope: str,
    owner: object,
    attribute: str,
    post_scan: Callable[[], None] | None = None,
) -> None:
    stack = getattr(owner, attribute, None)
    if stack is None:
        return
    if not isinstance(stack, list):
        raise HarnessIntegrityError(f"{scope} cleanup stack is not a list")
    for index, entry in enumerate(list(stack)):
        if not isinstance(entry, tuple) or len(entry) != 3:
            raise HarnessIntegrityError(f"{scope} cleanup entry has invalid shape")
        function, args, kwargs = entry
        if getattr(function, "__orays_audited_cleanup__", False):
            continue
        stack[index] = (
            audited_cleanup(
                audit,
                scope,
                function,
                post_call=(
                    post_scan
                    if post_scan is not None
                    else lambda: audit_cleanup_stack(audit, scope, owner, attribute)
                ),
            ),
            args,
            kwargs,
        )


def flatten_suite(suite: unittest.TestSuite) -> list[unittest.TestCase]:
    flattened: list[unittest.TestCase] = []
    for item in suite:
        if isinstance(item, unittest.TestSuite):
            flattened.extend(flatten_suite(item))
        elif isinstance(item, TRUSTED_TEST_CASE):
            flattened.append(item)
        else:
            raise HarnessIntegrityError(
                f"trusted unittest discovery produced unsupported item {type(item).__name__}"
            )
    return flattened


def discover_native_suite(
    module: types.ModuleType,
    inventory: list[tuple[str, str, int]],
    classes: dict[str, type[unittest.TestCase]],
) -> tuple[unittest.TestSuite, list[unittest.TestCase]]:
    forbidden_module_hooks = sorted(
        name for name in ("load_tests", "__getattr__", "__dir__") if name in module.__dict__
    )
    if forbidden_module_hooks:
        raise HarnessIntegrityError(
            f"canonical unittest exposes unsupported discovery hooks: {forbidden_module_hooks}"
        )
    loader = TRUSTED_TEST_LOADER()
    suite = loader.loadTestsFromModule(module)
    leaves = flatten_suite(suite)
    expected = Counter(
        (classes[class_name], method_name)
        for class_name, method_name, _line_number in inventory
    )
    observed = Counter(
        (type(test), getattr(test, "_testMethodName", ""))
        for test in leaves
    )
    if observed != expected or len(leaves) != len(inventory):
        expected_names = sorted(f"{case.__name__}.{method}" for case, method in expected)
        observed_names = sorted(f"{case.__name__}.{method}" for case, method in observed)
        raise HarnessIntegrityError(
            "trusted unittest discovery does not exactly match the canonical AST plan: "
            f"expected={expected_names}, observed={observed_names}"
        )
    return suite, leaves


def prepare_native_module_identity(
    module: types.ModuleType,
    module_name: str,
) -> None:
    for value in list(module.__dict__.values()):
        if getattr(value, "__module__", None) == module_name:
            try:
                value.__module__ = "__main__"
            except (AttributeError, TypeError):
                pass
    module.__dict__["__name__"] = "__main__"
    module.__dict__["__package__"] = None
    module.__dict__["__spec__"] = None


def wrap_lifecycle_hooks(
    audit: ExecutionAudit,
    module: types.ModuleType,
    classes: dict[str, type[unittest.TestCase]],
    inventory: list[tuple[str, str, int]],
    audit_all_cleanup_stacks: Callable[[], None],
) -> None:
    counts_by_class = Counter(class_name for class_name, _method, _line in inventory)
    for hook_name in ("setUpModule", "tearDownModule"):
        original = module.__dict__.get(hook_name)
        if original is None:
            continue
        if not isinstance(original, types.FunctionType):
            raise HarnessIntegrityError(f"module hook {hook_name} must be a plain function")
        identity = f"hook:module:{hook_name}"
        audit.expect(identity)

        @functools.wraps(original)
        def module_hook(
            *,
            _original: types.FunctionType = original,
            _identity: str = identity,
        ) -> None:
            invoke_checked(
                audit,
                _identity,
                _original,
                post_call=audit_all_cleanup_stacks,
            )

        module.__dict__[hook_name] = module_hook

    for class_name, test_class in classes.items():
        class_count = counts_by_class[class_name]
        for hook_name in ("setUp", "tearDown"):
            original = test_class.__dict__.get(hook_name)
            if original is None:
                continue
            if not isinstance(original, types.FunctionType):
                raise HarnessIntegrityError(
                    f"{class_name}.{hook_name} must be a plain synchronous method"
                )
            identity = f"hook:{class_name}:{hook_name}"
            audit.expect(identity, class_count)

            @functools.wraps(original)
            def instance_hook(
                instance: unittest.TestCase,
                *,
                _original: types.FunctionType = original,
                _identity: str = identity,
            ) -> None:
                invoke_checked(
                    audit,
                    _identity,
                    _original,
                    instance,
                    post_call=audit_all_cleanup_stacks,
                )

            setattr(test_class, hook_name, instance_hook)

        for hook_name in ("setUpClass", "tearDownClass"):
            descriptor = test_class.__dict__.get(hook_name)
            if descriptor is None:
                continue
            if not isinstance(descriptor, classmethod) or not isinstance(
                descriptor.__func__, types.FunctionType
            ):
                raise HarnessIntegrityError(
                    f"{class_name}.{hook_name} must be a plain @classmethod"
                )
            original = descriptor.__func__
            identity = f"hook:{class_name}:{hook_name}"
            scope = f"class:{class_name}"
            audit.expect(identity)

            @functools.wraps(original)
            def class_hook(
                cls: type[unittest.TestCase],
                *,
                _original: types.FunctionType = original,
                _identity: str = identity,
                _scope: str = scope,
            ) -> None:
                invoke_checked(
                    audit,
                    _identity,
                    _original,
                    cls,
                    post_call=audit_all_cleanup_stacks,
                )

            setattr(test_class, hook_name, classmethod(class_hook))


def execute_bound_suite(
    path: Path,
    module: types.ModuleType,
    module_name: str,
    inventory: list[tuple[str, str, int]],
    classes: dict[str, type[unittest.TestCase]],
) -> int:
    audit = ExecutionAudit()
    prepare_native_module_identity(module, module_name)
    previous_main = sys.modules.get("__main__")
    previous_argv = sys.argv[:]
    suite, leaves = discover_native_suite(module, inventory, classes)
    expected_ids = [test.id() for test in leaves]
    execution_counts: Counter[tuple[str, str]] = Counter()

    def audit_all_cleanup_stacks() -> None:
        for instance in leaves:
            audit_cleanup_stack(
                audit,
                f"instance:{instance.id()}",
                instance,
                "_cleanups",
                audit_all_cleanup_stacks,
            )
        for class_name, test_class in classes.items():
            audit_cleanup_stack(
                audit,
                f"class:{class_name}",
                test_class,
                "_class_cleanups",
                audit_all_cleanup_stacks,
            )
        audit_cleanup_stack(
            audit,
            "module",
            unittest.case,
            "_module_cleanups",
            audit_all_cleanup_stacks,
        )

    for class_name, method_name, _line_number in inventory:
        test_class = classes[class_name]
        original = test_class.__dict__[method_name]
        identity = (class_name, method_name)
        audit_identity = f"test:{class_name}.{method_name}"
        audit.expect(audit_identity)

        @functools.wraps(original)
        def bound_method(
            instance: unittest.TestCase,
            *,
            _original: types.FunctionType = original,
            _identity: tuple[str, str] = identity,
            _audit_identity: str = audit_identity,
        ) -> None:
            execution_counts[_identity] += 1
            invoke_checked(
                audit,
                _audit_identity,
                _original,
                instance,
                post_call=audit_all_cleanup_stacks,
            )

        setattr(test_class, method_name, bound_method)

    for instance in leaves:
        scope = f"instance:{instance.id()}"

        def add_cleanup(
            function: object,
            *args: object,
            _instance: unittest.TestCase = instance,
            _scope: str = scope,
            **kwargs: object,
        ) -> None:
            TRUSTED_TEST_CASE.addCleanup(
                _instance,
                audited_cleanup(
                    audit,
                    _scope,
                    function,
                    post_call=audit_all_cleanup_stacks,
                ),
                *args,
                **kwargs,
            )

        instance.addCleanup = add_cleanup  # type: ignore[method-assign]
        audit_all_cleanup_stacks()

    for class_name, test_class in classes.items():
        original_add_class_cleanup = TRUSTED_TEST_CASE.addClassCleanup.__func__

        def add_class_cleanup(
            cls: type[unittest.TestCase],
            function: object,
            *args: object,
            _scope: str = f"class:{class_name}",
            **kwargs: object,
        ) -> None:
            original_add_class_cleanup(
                cls,
                audited_cleanup(
                    audit,
                    _scope,
                    function,
                    post_call=audit_all_cleanup_stacks,
                ),
                *args,
                **kwargs,
            )

        setattr(test_class, "addClassCleanup", classmethod(add_class_cleanup))
        audit_all_cleanup_stacks()

    original_add_module_cleanup = TRUSTED_UNITTEST_BINDINGS["addModuleCleanup"]

    def add_module_cleanup(function: object, *args: object, **kwargs: object) -> None:
        original_add_module_cleanup(
            audited_cleanup(
                audit,
                "module",
                function,
                post_call=audit_all_cleanup_stacks,
            ),
            *args,
            **kwargs,
        )

    audit_all_cleanup_stacks()
    wrap_lifecycle_hooks(
        audit,
        module,
        classes,
        inventory,
        audit_all_cleanup_stacks,
    )

    expected_count = len(inventory)

    class AuditedBindingTextResult(BindingTextResult):
        def __init__(self, *args: Any, **kwargs: Any) -> None:
            super().__init__(*args, **kwargs)
            self.binding_emitted = False

        def stopTestRun(self) -> None:
            audit_all_cleanup_stacks()
            observed_counts = [
                execution_counts[(class_name, method_name)]
                for class_name, method_name, _line_number in inventory
            ]
            if (
                self.testsRun == expected_count
                and self.started_ids == expected_ids
                and self.stopped_ids == expected_ids
                and observed_counts == [1] * expected_count
                and not self.outcome_events
                and self.wasSuccessful()
                and audit.complete()
            ):
                self.stream.writeln(
                    "UNITTEST_BINDING: "
                    f"planned={expected_count} started={len(self.started_ids)} "
                    f"executed={sum(observed_counts)} stopped={len(self.stopped_ids)}"
                )
                self.binding_emitted = True
            super().stopTestRun()

    runner = TRUSTED_TEXT_RUNNER(
        stream=sys.stderr,
        verbosity=1,
        resultclass=AuditedBindingTextResult,
    )
    sys.modules["__main__"] = module
    sys.argv = [str(path)]
    unittest.addModuleCleanup = add_module_cleanup
    module_cleanup_binding_tampered = False
    try:
        with FileDescriptorTee() as captured:
            result = runner.run(suite)
    finally:
        module_cleanup_binding_tampered = unittest.addModuleCleanup is not add_module_cleanup
        unittest.addModuleCleanup = original_add_module_cleanup
        sys.argv = previous_argv
        if previous_main is None:
            sys.modules.pop("__main__", None)
        else:
            sys.modules["__main__"] = previous_main

    if module_cleanup_binding_tampered:
        print(
            "UNITTEST_HARNESS_ERROR: test module mutated unittest.addModuleCleanup during execution",
            file=sys.stderr,
        )
        return 2

    try:
        verify_framework_unchanged()
    except HarnessIntegrityError as error:
        sys.stderr.write(f"UNITTEST_HARNESS_ERROR: {error}\n")
        return 2

    observed_execution_counts = [
        execution_counts[(class_name, method_name)]
        for class_name, method_name, _line_number in inventory
    ]
    binding_complete = (
        result.testsRun == len(inventory)
        and result.started_ids == expected_ids
        and result.stopped_ids == expected_ids
        and observed_execution_counts == [1] * len(inventory)
        and result.binding_emitted
        and audit.complete()
    )
    unsupported_outcomes = bool(
        result.skipped
        or result.expectedFailures
        or result.unexpectedSuccesses
        or result.outcome_events
    )

    if not result.wasSuccessful() or unsupported_outcomes:
        if unsupported_outcomes:
            sys.stderr.write(
                "UNITTEST_HARNESS_ERROR: non-success outcome events are not complete success\n"
            )
        return 1
    if not binding_complete:
        sys.stderr.write(
            "UNITTEST_HARNESS_ERROR: runtime identity binding mismatch: "
            f"planned={len(inventory)} started={len(result.started_ids)} "
            f"executed={sum(observed_execution_counts)} stopped={len(result.stopped_ids)}\n"
        )
        return 2
    if captured.stdout_bytes():
        sys.stderr.write(
            "UNITTEST_HARNESS_ERROR: a passing canonical unittest wrote to stdout\n"
        )
        return 2
    return 0


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("implementation", type=Path)
    args = parser.parse_args(argv)
    path = args.implementation.expanduser().resolve()
    module_name: str | None = None
    try:
        source, tree = strict_source(path)
        inventory = suite_contract.canonical_unittest_inventory(tree, path)
        module, module_name, _import_output = load_module(path, source)
        classes = runtime_classes(module, module_name, path, inventory)
        return execute_bound_suite(path, module, module_name, inventory, classes)
    except (HarnessIntegrityError, suite_contract.ManifestError) as error:
        print(f"UNITTEST_HARNESS_ERROR: {error}", file=sys.stderr)
        return 2
    finally:
        if module_name is not None:
            sys.modules.pop(module_name, None)


if __name__ == "__main__":
    raise SystemExit(main())
