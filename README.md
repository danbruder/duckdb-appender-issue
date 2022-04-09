Minimal reproducible example for timestamp field in appender issue

Error at commit 2: 
```
Caused by:
  process didn't exit successfully: `/Users/dan/oss/duck-mem/target/debug/deps/duck_mem-1fa8cbab329fd802 timestamp_appender_minimal_example_sig_segv` (signal: 11, SIGSEGV: invalid memory reference)
```


Error at commit 1: 

```
duck-mem(77033,0x11c704e00) malloc: *** error for object 0x7fac8f406ae0: pointer being freed was not allocated
duck-mem(77033,0x11c704e00) malloc: *** set a breakpoint in malloc_error_break to debug
```

