; ModuleID = 'probe4.fe5216d0a62e3dcd-cgu.0'
source_filename = "probe4.fe5216d0a62e3dcd-cgu.0"
target datalayout = "e-m:e-i8:8:32-i16:16:32-i64:64-i128:128-n32:64-S128"
target triple = "aarch64-alpine-linux-musl"

@alloc_89290aebf0806bac9cdbd56e82737aa8 = private unnamed_addr constant <{ [86 x i8] }> <{ [86 x i8] c"/home/buildozer/aports/community/rust/src/rustc-1.71.1-src/library/core/src/num/mod.rs" }>, align 1
@alloc_7509be482a15d715d81cd29f2b127abb = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_89290aebf0806bac9cdbd56e82737aa8, [16 x i8] c"V\00\00\00\00\00\00\00~\04\00\00\05\00\00\00" }>, align 8
@str.0 = internal constant [25 x i8] c"attempt to divide by zero"

; probe4::probe
; Function Attrs: nonlazybind uwtable
define void @_ZN6probe45probe17hd645267811c80123E() unnamed_addr #0 {
start:
  %0 = call i1 @llvm.expect.i1(i1 false, i1 false)
  br i1 %0, label %panic.i, label %"_ZN4core3num21_$LT$impl$u20$u32$GT$10div_euclid17h31f226e2628856beE.exit"

panic.i:                                          ; preds = %start
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17hd50cd8b22eba6389E(ptr align 1 @str.0, i64 25, ptr align 8 @alloc_7509be482a15d715d81cd29f2b127abb) #3
  unreachable

"_ZN4core3num21_$LT$impl$u20$u32$GT$10div_euclid17h31f226e2628856beE.exit": ; preds = %start
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind willreturn memory(none)
declare i1 @llvm.expect.i1(i1, i1) #1

; core::panicking::panic
; Function Attrs: cold noinline noreturn nonlazybind uwtable
declare void @_ZN4core9panicking5panic17hd50cd8b22eba6389E(ptr align 1, i64, ptr align 8) unnamed_addr #2

attributes #0 = { nonlazybind uwtable "target-cpu"="generic" "target-features"="+v8a" }
attributes #1 = { nocallback nofree nosync nounwind willreturn memory(none) }
attributes #2 = { cold noinline noreturn nonlazybind uwtable "target-cpu"="generic" "target-features"="+v8a" }
attributes #3 = { noreturn }

!llvm.module.flags = !{!0, !1}

!0 = !{i32 8, !"PIC Level", i32 2}
!1 = !{i32 2, !"RtLibUseGOT", i32 1}
