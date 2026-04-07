; WispyScheme algebra library for Stak VM
; Cayley table constants and native algebra primitives (dot, tau, type-valid?)

(define-library (wispy algebra)
  (export
    TOP BOT Q E
    CAR CDR CONS RHO
    APPLY CC TAU Y
    T_PAIR T_SYM T_CLS T_STR T_VEC T_CHAR T_CONT T_PORT
    TRUE EOF VOID
    T_RECORD T_VALUES T_ERROR T_BYTEVEC T_PROMISE
    dot tau type-valid?)

  (import (scheme base))

  (begin
    ;; Algebraic core (0-11)
    (define TOP 0)
    (define BOT 1)
    (define Q 2)
    (define E 3)
    (define CAR 4)
    (define CDR 5)
    (define CONS 6)
    (define RHO 7)
    (define APPLY 8)
    (define CC 9)
    (define TAU 10)
    (define Y 11)

    ;; R4RS type tags (12-19)
    (define T_PAIR 12)
    (define T_SYM 13)
    (define T_CLS 14)
    (define T_STR 15)
    (define T_VEC 16)
    (define T_CHAR 17)
    (define T_CONT 18)
    (define T_PORT 19)

    ;; Special values (20-22)
    (define TRUE 20)
    (define EOF 21)
    (define VOID 22)

    ;; R7RS type tags (23-27)
    (define T_RECORD 23)
    (define T_VALUES 24)
    (define T_ERROR 25)
    (define T_BYTEVEC 26)
    (define T_PROMISE 27)

    ;; Native algebra primitives (handled by WispyPrimitiveSet)
    ;; (rib id '() 3) creates a procedure rib with the given primitive ID
    (define dot (rib 600 '() 3))
    (define tau (rib 601 '() 3))
    (define type-valid? (rib 602 '() 3))))
