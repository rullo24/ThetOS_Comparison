#include "FreeRTOS.h"
#include "task.h"
#include "stm32l1xx_hal.h"

static TIM_HandleTypeDef htim;

/* A FreeRTOS task starts PWM on a timer that was never initialised. */
void vTask( void * pvParameters )
{
    ( void ) pvParameters;

    HAL_TIM_PWM_Start( &htim, TIM_CHANNEL_1 ); /* HAL_TIM_PWM_Init() was never called */

    for( ;; ) { vTaskDelay( 10 ); }
}
