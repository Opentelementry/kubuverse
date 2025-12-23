#define PY_SSIZE_T_CLEAN
#include <Python.h>
#include <windows.h>
 
static PyObject* kv_get_overlapped_result(PyObject* self, PyObject* args) {
    PyObject* py_handle;
    int wait;
    if (!PyArg_ParseTuple(args, "Op", &py_handle, &wait))
        return NULL;

    HANDLE h = PyLong_AsVoidPtr(py_handle);
    DWORD bytesTransferred = 0;
    BOOL result = GetOverlappedResult(h, NULL, &bytesTransferred, wait);

    if (!result)
        return PyErr_SetFromWindowsErr(0);

    return PyLong_FromUnsignedLong(bytesTransferred);
}

static PyMethodDef KubuWinMethods[] = {
    {"get_overlapped_result", kv_get_overlapped_result, METH_VARARGS,
     "Custom Kubuverse Overlapped I/O handler"},
    {NULL, NULL, 0, NULL}
};

static struct PyModuleDef kubu_winapi_module = {
    PyModuleDef_HEAD_INIT, "kubu_winapi", NULL, -1, KubuWinMethods
};

PyMODINIT_FUNC PyInit_kubu_winapi(void) {
    return PyModule_Create(&kubu_winapi_module);
}
