# cmakefmt: push { commandCase = "upper", spaceBeforeParen = ["if",], perCommandConfig = { set = { wrapStyle = "vertical", }, }, }
if(TRUE)
  set(MY_VAR val1 val2 val3 val4 val5 val6 val7 val8 val9 val10 val11 val12)
endif()
# cmakefmt: pop
