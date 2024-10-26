import React, { useState } from 'react';
import { Bar } from 'react-chartjs-2';
import { Box, Button, ButtonGroup, Text, Flex } from '@chakra-ui/react';
import {
    Chart as ChartJS,
    CategoryScale,
    LinearScale,
    BarElement,
    Title,
    Tooltip,
    Legend
} from 'chart.js';
import { useAllInfo } from '../../store'

ChartJS.register(
    CategoryScale,
    LinearScale,
    BarElement,
    Title,
    Tooltip,
    Legend
);

const CurvedLineChart = ({ data, options }) => {
    return (
        <Box pl='6' width="100%" display='flex' justifyContent='center'>
            <Bar data={data} options={options} />
        </Box>
    );
};

const ChartContainer = () => {
    const [timeUnit, setTimeUnit] = useState('5D');

    const { users, userslist } = useAllInfo();
    const data = {
        labels: getLabelsBasedOnTimeUnit(timeUnit),
        datasets: [
            {
                label: 'User',
                data: getDataBasedOnTimeUnit(timeUnit),
                backgroundColor: '#E9983D',
                borderRadius: 20,
                barThickness: 8,
            },
        ],
    };

    const options = {
        responsive: true,
        plugins: {
            legend: {
                display: false,
            },
            title: {
                display: true,
            },
        },
        scales: {
            x: {
                grid: {
                    display: false, // Disable x-axis grid lines
                },
            },
            y: {
                display: false,
            },
        },
    };

    function getLabelsBasedOnTimeUnit(unit) {
        if (unit === '1D') {
            return ['0h', '4h', '8h', '12h', '16h', '20h'];
        } else if (unit === '5D') {
            const labels = [];
            const today = new Date();
            labels.push(formatDate(today));


            for (let i = 1; i < 5; i++) {
                const previousDay = new Date(today);
                previousDay.setDate(today.getDate() - i);
                labels.push(formatDate(previousDay));
            }

            return labels.reverse();
        }
    }


    function getDataBasedOnTimeUnit(unit) {
        if (unit === '1D') {
            return [12, 19, 3, 5, 2, 100];
        } else if (unit === '5D') {
            return userslist;
        }
    }
    function formatDate(date) {
        const options = { month: 'short', day: 'numeric', year: 'numeric' };
        return date.toLocaleDateString('en-US', options);
    }
    const currentTime = new Date();
    const test = () => {
        console.log(userslist)
    }
    return (
        <Box width="50%" >
            <Flex justifyContent="space-between" alignItems="center" mb="2" p='6' pl='6'>
                <Flex width='100%' ml="6" direction='column'>
                    <Flex justifyContent="space-between" alignItems="center">
                        <Text fontSize={{ md: '12px', lg: '12px', xl: '12px', '2xl': '12px', '3xl': '12px' }}>Total User(24h)</Text>
                        {/*
                        <ButtonGroup variant="outline">
                            <Button
                                size={{ md: 'sm', lg: 'sm', xl: 'md', '2xl': 'md' }}
                                borderRadius='3xl'
                                onClick={() => setTimeUnit('1D')}
                                bg={timeUnit === '1D' ? '#3861FB' : 'gray.200'}
                                color={timeUnit === '1D' ? 'white' : 'black'}
                            >
                                1D
                            </Button>
                            <Button
                                size={{ md: 'sm', lg: 'sm', xl: 'md', '2xl': 'md' }}
                                borderRadius='3xl'
                                onClick={() => setTimeUnit('5D')}
                                bg={timeUnit === '5D' ? '#3861FB' : 'gray.200'}
                                color={timeUnit === '5D' ? 'white' : 'black'}
                            >
                                5D
                            </Button>
                        </ButtonGroup>
                    */}
                    </Flex>
                    {/*
                        <Button onClick={test}>test</Button>
                    */}
                    <Text fontSize={{ md: '40px', lg: '40px', xl: '40px', '2xl': '40px', '3xl': '40px' }} fontWeight='bold'>{users}</Text>
                    <Text fontSize={{ md: '12px', lg: '12px', xl: '12px', '2xl': '12px', '3xl': '12px' }}>{formatDate(currentTime)}</Text>
                </Flex>

            </Flex>
            <CurvedLineChart data={data} options={options} />
        </Box>
    );
};

export default ChartContainer;