# cmakefmt: push { alignTrailingComments = true }
set(A "1") # outer
if(TRUE)
  set(B "2")      # inner
  set(LONG_C "3") # inner
endif()
# cmakefmt: pop
