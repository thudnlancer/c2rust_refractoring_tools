; ModuleID = 'autocfg_dcf437a1c9e822e3_1.ea01dc87-cgu.0'
source_filename = "autocfg_dcf437a1c9e822e3_1.ea01dc87-cgu.0"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

%"[closure@core::f64::<impl f64>::to_bits::{closure#0}]" = type {}
%"unwind::libunwind::_Unwind_Exception" = type { i64, void (i32, %"unwind::libunwind::_Unwind_Exception"*)*, [6 x i64] }
%"unwind::libunwind::_Unwind_Context" = type { [0 x i8] }

@alloc6 = private unnamed_addr constant <{ [8 x i8] }> <{ [8 x i8] c"\00\00\00\00\00\00\F0?" }>, align 8
@alloc8 = private unnamed_addr constant <{ [8 x i8] }> <{ [8 x i8] c"\00\00\00\00\00\00\00@" }>, align 8

; core::intrinsics::const_eval_select
; Function Attrs: inlinehint nonlazybind uwtable
define i64 @_ZN4core10intrinsics17const_eval_select17h6bafb0de8f3f2cb8E(double %arg) unnamed_addr #0 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
start:
  %0 = alloca { i8*, i32 }, align 8
; invoke core::ops::function::FnOnce::call_once
  %1 = invoke i64 @_ZN4core3ops8function6FnOnce9call_once17h836f0d0df00acac1E(double %arg)
          to label %bb1 unwind label %cleanup

bb3:                                              ; preds = %cleanup
  br label %bb4

cleanup:                                          ; preds = %start
  %2 = landingpad { i8*, i32 }
          cleanup
  %3 = extractvalue { i8*, i32 } %2, 0
  %4 = extractvalue { i8*, i32 } %2, 1
  %5 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %0, i32 0, i32 0
  store i8* %3, i8** %5, align 8
  %6 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %0, i32 0, i32 1
  store i32 %4, i32* %6, align 8
  br label %bb3

bb1:                                              ; preds = %start
  br label %bb2

bb4:                                              ; preds = %bb3
  %7 = bitcast { i8*, i32 }* %0 to i8**
  %8 = load i8*, i8** %7, align 8
  %9 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %0, i32 0, i32 1
  %10 = load i32, i32* %9, align 8
  %11 = insertvalue { i8*, i32 } undef, i8* %8, 0
  %12 = insertvalue { i8*, i32 } %11, i32 %10, 1
  resume { i8*, i32 } %12

bb2:                                              ; preds = %bb1
  ret i64 %1
}

; core::f64::<impl f64>::to_bits::{{closure}}
; Function Attrs: inlinehint nonlazybind uwtable
define i64 @"_ZN4core3f6421_$LT$impl$u20$f64$GT$7to_bits28_$u7b$$u7b$closure$u7d$$u7d$17hb80a8b2865efa94dE"(%"[closure@core::f64::<impl f64>::to_bits::{closure#0}]"* align 1 %_1, double %rt) unnamed_addr #0 {
start:
  %0 = alloca i64, align 8
  %1 = bitcast double %rt to i64
  store i64 %1, i64* %0, align 8
  %2 = load i64, i64* %0, align 8
  br label %bb1

bb1:                                              ; preds = %start
  ret i64 %2
}

; core::f64::<impl f64>::total_cmp
; Function Attrs: inlinehint nonlazybind uwtable
define internal i8 @"_ZN4core3f6421_$LT$impl$u20$f64$GT$9total_cmp17h5d7c8bba39902394E"(double* align 8 %self, double* align 8 %other) unnamed_addr #0 {
start:
  %_27 = alloca double, align 8
  %_23 = alloca double, align 8
  %right = alloca i64, align 8
  %left = alloca i64, align 8
  %0 = alloca i8, align 1
  %self1 = load double, double* %self, align 8
  store double %self1, double* %_23, align 8
  %1 = load double, double* %_23, align 8
; call core::intrinsics::const_eval_select
  %_4 = call i64 @_ZN4core10intrinsics17const_eval_select17h6bafb0de8f3f2cb8E(double %1)
  br label %bb1

bb1:                                              ; preds = %start
  store i64 %_4, i64* %left, align 8
  %self2 = load double, double* %other, align 8
  store double %self2, double* %_27, align 8
  %2 = load double, double* %_27, align 8
; call core::intrinsics::const_eval_select
  %_7 = call i64 @_ZN4core10intrinsics17const_eval_select17h6bafb0de8f3f2cb8E(double %2)
  br label %bb2

bb2:                                              ; preds = %bb1
  store i64 %_7, i64* %right, align 8
  %_13 = load i64, i64* %left, align 8
  %_12 = ashr i64 %_13, 63
  %_10 = lshr i64 %_12, 1
  %3 = load i64, i64* %left, align 8
  %4 = xor i64 %3, %_10
  store i64 %4, i64* %left, align 8
  %_18 = load i64, i64* %right, align 8
  %_17 = ashr i64 %_18, 63
  %_15 = lshr i64 %_17, 1
  %5 = load i64, i64* %right, align 8
  %6 = xor i64 %5, %_15
  store i64 %6, i64* %right, align 8
  %_31 = load i64, i64* %left, align 8
  %_32 = load i64, i64* %right, align 8
  %_30 = icmp slt i64 %_31, %_32
  br i1 %_30, label %bb3, label %bb4

bb4:                                              ; preds = %bb2
  %_34 = load i64, i64* %left, align 8
  %_35 = load i64, i64* %right, align 8
  %_33 = icmp eq i64 %_34, %_35
  br i1 %_33, label %bb5, label %bb6

bb3:                                              ; preds = %bb2
  store i8 -1, i8* %0, align 1
  br label %bb8

bb8:                                              ; preds = %bb7, %bb3
  %7 = load i8, i8* %0, align 1, !range !2, !noundef !3
  ret i8 %7

bb6:                                              ; preds = %bb4
  store i8 1, i8* %0, align 1
  br label %bb7

bb5:                                              ; preds = %bb4
  store i8 0, i8* %0, align 1
  br label %bb7

bb7:                                              ; preds = %bb6, %bb5
  br label %bb8
}

; core::ops::function::FnOnce::call_once
; Function Attrs: inlinehint nonlazybind uwtable
define internal i64 @_ZN4core3ops8function6FnOnce9call_once17h836f0d0df00acac1E(double %0) unnamed_addr #0 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
start:
  %1 = alloca { i8*, i32 }, align 8
  %_2 = alloca double, align 8
  %_1 = alloca %"[closure@core::f64::<impl f64>::to_bits::{closure#0}]", align 1
  store double %0, double* %_2, align 8
  %2 = load double, double* %_2, align 8
; invoke core::f64::<impl f64>::to_bits::{{closure}}
  %3 = invoke i64 @"_ZN4core3f6421_$LT$impl$u20$f64$GT$7to_bits28_$u7b$$u7b$closure$u7d$$u7d$17hb80a8b2865efa94dE"(%"[closure@core::f64::<impl f64>::to_bits::{closure#0}]"* align 1 %_1, double %2)
          to label %bb1 unwind label %cleanup

bb3:                                              ; preds = %cleanup
  br label %bb4

cleanup:                                          ; preds = %start
  %4 = landingpad { i8*, i32 }
          cleanup
  %5 = extractvalue { i8*, i32 } %4, 0
  %6 = extractvalue { i8*, i32 } %4, 1
  %7 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %1, i32 0, i32 0
  store i8* %5, i8** %7, align 8
  %8 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %1, i32 0, i32 1
  store i32 %6, i32* %8, align 8
  br label %bb3

bb1:                                              ; preds = %start
  br label %bb2

bb4:                                              ; preds = %bb3
  %9 = bitcast { i8*, i32 }* %1 to i8**
  %10 = load i8*, i8** %9, align 8
  %11 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %1, i32 0, i32 1
  %12 = load i32, i32* %11, align 8
  %13 = insertvalue { i8*, i32 } undef, i8* %10, 0
  %14 = insertvalue { i8*, i32 } %13, i32 %12, 1
  resume { i8*, i32 } %14

bb2:                                              ; preds = %bb1
  ret i64 %3
}

; autocfg_dcf437a1c9e822e3_1::probe
; Function Attrs: nonlazybind uwtable
define void @_ZN26autocfg_dcf437a1c9e822e3_15probe17hecab88e60ba63d40E() unnamed_addr #1 {
start:
; call core::f64::<impl f64>::total_cmp
  %_1 = call i8 @"_ZN4core3f6421_$LT$impl$u20$f64$GT$9total_cmp17h5d7c8bba39902394E"(double* align 8 bitcast (<{ [8 x i8] }>* @alloc6 to double*), double* align 8 bitcast (<{ [8 x i8] }>* @alloc8 to double*)), !range !2
  br label %bb1

bb1:                                              ; preds = %start
  ret void
}

; Function Attrs: nonlazybind uwtable
declare i32 @rust_eh_personality(i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*) unnamed_addr #1

attributes #0 = { inlinehint nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" }
attributes #1 = { nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" }

!llvm.module.flags = !{!0, !1}

!0 = !{i32 7, !"PIC Level", i32 2}
!1 = !{i32 2, !"RtLibUseGOT", i32 1}
!2 = !{i8 -1, i8 2}
!3 = !{}
