#include "FreeRTOS.h"
#include "task.h"

/* Two tasks share a plain variable with no synchronisation. */
static unsigned short duty;

static void writer_task( void * pvParameters )
{
    ( void ) pvParameters;
    for( ;; ) { duty = 100; vTaskDelay( 10 ); }
}

static void reader_task( void * pvParameters )
{
    ( void ) pvParameters;
    for( ;; ) { volatile unsigned short d = duty; ( void ) d; vTaskDelay( 10 ); }
}

void create_tasks( void )
{
    xTaskCreate( writer_task, "w", configMINIMAL_STACK_SIZE, NULL, 1, NULL );
    xTaskCreate( reader_task, "r", configMINIMAL_STACK_SIZE, NULL, 1, NULL );
}
