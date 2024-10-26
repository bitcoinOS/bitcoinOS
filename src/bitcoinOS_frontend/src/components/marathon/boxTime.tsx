import React, { useState, useEffect, useCallback } from 'react';
import { Box, Text, Button } from '@chakra-ui/react';
import { boxStore } from '../../store/useMarathonStore';
import useGetMarathon from '../../utils/marathonActor';

function BoxTime() {
    const [remainingTime, setRemainingTime] = useState(boxStore.getState().remainingTimes[0] || 0);
    const updateRemainingTime = boxStore(state => state.updateRemainingTime);
    const { update_login } = useGetMarathon();

    const decrementTime = useCallback(() => {
        setRemainingTime(prevTime => {
            const newTime = Math.max(prevTime - 1, 0);
            updateRemainingTime(0, newTime);
            return newTime;
        });
    }, [updateRemainingTime]);

    useEffect(() => {
        if (remainingTime <= 0) return;

        const timer = setInterval(decrementTime, 1000); // Update every second

        return () => clearInterval(timer);
    }, [remainingTime, decrementTime]);

    const formatTime = (totalSeconds) => {
        const hours = Math.floor(totalSeconds / 3600);
        const minutes = Math.floor((totalSeconds % 3600) / 60);
        const seconds = totalSeconds % 60;
        return `${hours.toString().padStart(2, '0')}:${minutes.toString().padStart(2, '0')}:${seconds.toString().padStart(2, '0')}`;
    };

    return (
        <Box>
            <Text
                width='128px'
                height='40px'
                borderRadius='8px'
                fontSize='16px'
                fontWeight='700'
                bgColor='#E0E0E0'
                textAlign='center'
                display='flex'
                alignItems='center'
                justifyContent='center'
            >
                {formatTime(Math.floor(remainingTime))}
            </Text>
        </Box>
    );
}

export default BoxTime;