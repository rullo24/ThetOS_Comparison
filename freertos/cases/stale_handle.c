#include "FreeRTOS.h"
#include "task.h"
#include "stm32l1xx_hal.h"

static UART_HandleTypeDef huart;

/* A FreeRTOS task keeps using a UART handle after the peripheral was de-initialised. */
void vTask( void * pvParameters )
{
    ( void ) pvParameters;
    uint8_t byte = 'x';

    HAL_UART_Init( &huart );
    HAL_UART_DeInit( &huart );
    HAL_UART_Transmit( &huart, &byte, 1, 100 ); /* handle is stale after DeInit */

    for( ;; ) { vTaskDelay( 10 ); }
}
