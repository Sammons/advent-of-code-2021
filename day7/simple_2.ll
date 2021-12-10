
@.str = private unnamed_addr constant [12 x i8] c"hello world\00", align 1

define i32 @main() #0 {
  %1 = call i32 @puts(i8* getelementptr inbounds ([12 x i8], [12 x i8]* @.str, i64 0, i64 0))
  ret i32 0
}

declare i32 @puts(i8*)
