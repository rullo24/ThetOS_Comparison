#include "FreeRTOS.h"
#include "task.h"
#include "stm32l1xx_hal.h"

/* A FreeRTOS task configures a pin as an INPUT, then writes to it. */
void vTask( void * pvParameters )
{
    ( void ) pvParameters;

    GPIO_InitTypeDef cfg = { 0 };
    cfg.Pin = GPIO_PIN_6;
    cfg.Mode = GPIO_MODE_INPUT;
    cfg.Pull = GPIO_NOPULL;
    HAL_GPIO_Init( GPIOA, &cfg );

    HAL_GPIO_WritePin( GPIOA, GPIO_PIN_6, GPIO_PIN_SET ); /* write to an input pin */

    for( ;; ) { vTaskDelay( 10 ); }
}
