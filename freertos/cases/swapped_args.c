#include "FreeRTOS.h"
#include "task.h"

static void task( void * pvParameters )
{
    ( void ) pvParameters;
    for( ;; ) { }
}

/* usStackDepth and uxPriority are both plain integers, so swapping them is accepted. */
void create_tasks( void )
{
    xTaskCreate( task, "t", 1, NULL, configMINIMAL_STACK_SIZE, NULL );
}
