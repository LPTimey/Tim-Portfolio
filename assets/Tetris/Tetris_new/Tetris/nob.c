// nob.c
#ifndef ARDUINO
#ifdef _WIN32
#include "windows.h"
#define PATH_SEP "\\"
#define EXT ".exe"
#else
#define PATH_SEP "/"
#define EXT ""
#endif /* ifdef _WIN32 */

#define NOB_IMPLEMENTATION
#include "nob.h"

#define NAME "main"
#define SRC "src"
#define OUT_ "out"

#define EXE OUT_ PATH_SEP NAME EXT

int main(int argc, char **argv) {
  NOB_GO_REBUILD_URSELF(argc, argv);
  Nob_Cmd cmd = {0};
  nob_cmd_append(&cmd, "clang++", "-g", "-Wall", "-Wextra", "-std=c++20", "-o", EXE,
                 SRC PATH_SEP "main.cpp");
  if (!nob_cmd_run_sync_and_reset(&cmd))
    return 1;

  nob_cmd_append(&cmd, "." PATH_SEP EXE);
  if (!nob_cmd_run_sync(cmd))
    return 1;
  return 0;
}
#endif