;; WispyScheme REPL — Cayley table algebra on Stak VM
;;
;; Pre-loads (wispy algebra) into the interaction environment so that
;; dot, tau, type-valid?, and all Cayley constants are available at the prompt.

(import
  (scheme base)
  (scheme eval)
  (scheme read)
  (scheme repl)
  (scheme write)
  (wispy algebra))

;; Seed the interaction environment with the algebra
(eval '(import (scheme base) (scheme write) (wispy algebra))
      (interaction-environment))

(define (write-value value)
  (if (error-object? value)
    (begin
      (display "ERROR: ")
      (display (error-object-message value))
      (for-each
        (lambda (value)
          (write-char #\space)
          (write value))
        (error-object-irritants value)))
    (write value)))

(define (prompt)
  (display "wispy> " (current-error-port))
  (guard
    (error
      (else
        (read-line)
        error))
    (read)))

(define (main)
  (display "WispyScheme REPL (Stak VM)\n" (current-error-port))
  (display "Cayley algebra: dot, tau, type-valid?, TOP..VOID\n" (current-error-port))
  (do ((expression (prompt) (prompt)))
    ((eof-object? expression))
    (write-value
      (if (error-object? expression)
        expression
        (guard (error (else error))
          (eval expression (interaction-environment)))))
    (newline)))

(main)
