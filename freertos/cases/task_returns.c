#include "FreeRTOS.h"
#include "task.h"

/* A task function that returns. FreeRTOS requires tasks never to return, but the type allows it. */
static void task( void * pvParameters )
{
    ( void ) pvParameters;
}

void create_tasks( void )
{
    xTaskCreate( task, "t", configMINIMAL_STACK_SIZE, NULL, 1, NULL );
}
