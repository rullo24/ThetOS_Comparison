#ifndef FREERTOS_CONFIG_H
#define FREERTOS_CONFIG_H

/* Minimal configuration: just enough for the kernel headers to compile for Cortex-M3.
 * Values mirror the STM32L152RET6 target used in the thesis (MSI clock, 4.194 MHz). */

#define configUSE_PREEMPTION                    1
#define configUSE_TIME_SLICING                  1
#define configUSE_IDLE_HOOK                     0
#define configUSE_TICK_HOOK                     0
#define configUSE_MALLOC_FAILED_HOOK            0
#define configUSE_TRACE_FACILITY                0
#define configUSE_MUTEXES                       1
#define configUSE_RECURSIVE_MUTEXES             0
#define configUSE_COUNTING_SEMAPHORES           0
#define configUSE_TIMERS                        0
#define configUSE_PORT_OPTIMISED_TASK_SELECTION 0
#define configCPU_CLOCK_HZ                      ( ( unsigned long ) 4194304 )
#define configTICK_RATE_HZ                      ( ( TickType_t ) 1000 )
#define configMAX_PRIORITIES                    ( 5 )
#define configMINIMAL_STACK_SIZE                ( ( unsigned short ) 128 )
#define configTOTAL_HEAP_SIZE                   ( ( size_t ) ( 8 * 1024 ) )
#define configMAX_TASK_NAME_LEN                 ( 16 )
#define configUSE_16_BIT_TICKS                  0
#define configIDLE_SHOULD_YIELD                 1
#define configSUPPORT_DYNAMIC_ALLOCATION        1
#define configSUPPORT_STATIC_ALLOCATION         0
#define configKERNEL_INTERRUPT_PRIORITY         ( 15 << 4 )
#define configMAX_SYSCALL_INTERRUPT_PRIORITY    ( 5 << 4 )
#define configPRIO_BITS                         4
#define configCHECK_FOR_STACK_OVERFLOW          2
#define configASSERT( x )                       do { if( ( x ) == 0 ) { for( ;; ); } } while( 0 )
#define INCLUDE_vTaskPrioritySet                1
#define INCLUDE_uxTaskPriorityGet               1
#define INCLUDE_vTaskDelete                     1
#define INCLUDE_vTaskSuspend                    1
#define INCLUDE_vTaskDelay                      1
#define INCLUDE_xTaskGetSchedulerState          1

#endif
