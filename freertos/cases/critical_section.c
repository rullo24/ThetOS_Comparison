#include "FreeRTOS.h"
#include "task.h"

/* The early return leaves the critical section entered; nothing in the language or API prevents it. */
int work( int early )
{
    taskENTER_CRITICAL();
    if( early )
    {
        return 1;
    }
    taskEXIT_CRITICAL();
    return 2;
}
