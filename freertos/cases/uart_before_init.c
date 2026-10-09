#include "FreeRTOS.h"
#include "task.h"
#include "stm32l1xx_hal.h"

static UART_HandleTypeDef huart;

/* A FreeRTOS task transmits on a UART that was never initialised. */
void vTask( void * pvParameters )
{
    ( void ) pvParameters;
    uint8_t byte = 'x';

    HAL_UART_Transmit( &huart, &byte, 1, 100 ); /* HAL_UART_Init() was never called */

    for( ;; ) { vTaskDelay( 10 ); }
}
