import React, { useState } from 'react';
import { Line } from 'react-chartjs-2';
import { Box, Button, ButtonGroup, Text, Flex } from '@chakra-ui/react';
import {
    Chart as ChartJS,
    CategoryScale,
    LinearScale,
    PointElement,
    LineElement,
    Title,
    Tooltip,
    Legend
} from 'chart.js';

ChartJS.register(
    CategoryScale,
    LinearScale,
    PointElement,
    LineElement,
    Title,
    Tooltip,
    Legend
);

import { useAllInfo } from '../../store';
const CurvedLineChart = ({ data, options }) => {
    return (
        <Box width="100%" >
            <Line data={data} options={options} />
        </Box>
    );
};

const ChartContainer = () => {
    const [timeUnit, setTimeUnit] = useState('5D');

    const { tvl, tvllist } = useAllInfo();
    const data = {
        labels: getLabelsBasedOnTimeUnit(timeUnit),
        datasets: [
            {
                label: 'TVL',
                data: getDataBasedOnTimeUnit(timeUnit),
                fill: false,
                backgroundColor: 'rgba(75,192,192,0.2)',
                borderColor: 'rgba(75,192,192,1)',
                tension: 0.4, // This value makes the line curved
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
            tooltip: {
                callbacks: {
                    label: function (context) {
                        let label = context.dataset.label || '';
                        if (label) {
                            label += ': ';
                        }
                        if (context.parsed.y !== null) {
                            label += context.parsed.y.toFixed(8);  // Display full 8 decimal places
                        }
                        return label;
                    }
                }
            }
        },
        scales: {
            x: {
                grid: {
                    display: false, // Disable x-axis grid lines
                },
            },
            y: {
                display: false,
                ticks: {
                    callback: function (value, index, values) {
                        return value.toFixed(8);  // Display full 8 decimal places on y-axis if it becomes visible
                    }
                }
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
            return [12, 19, 3, 5, 2, 3];
        } else if (unit === '5D') {
            return tvllist;
        }
    }

    function formatDate(date) {
        const options = { month: 'short', day: 'numeric', year: 'numeric' };
        return date.toLocaleDateString('en-US', options);
    }
    const currentTime = new Date();

    return (
        <Box bg="rgba(233, 235, 255, 0.3)" width="50%" p="6" borderRadius="md">
            <Flex justifyContent="space-between" mb="2">
                <Flex width='100%' direction='column'>
                    <Flex justifyContent="space-between" alignItems="center">
                        <Text fontSize={{ md: '12px', lg: '12px', xl: '12px', '2xl': '12px', '3xl': '12px' }} mr="2">Total Value Locked (btc)</Text>
                        {/*
                        <ButtonGroup variant="outline">
                            <Button
                                borderRadius='3xl'
                                size={{ md: 'sm', lg: 'sm', xl: 'md', '2xl': 'md' }}
                                onClick={() => setTimeUnit('1D')}
                                bg={timeUnit === '1D' ? '#3861FB' : 'gray.200'}
                                color={timeUnit === '1D' ? 'white' : 'black'}
                            >
                                1D
                            </Button>
                            <Button
                                borderRadius='3xl'
                                size={{ md: 'sm', lg: 'sm', xl: 'md', '2xl': 'md' }}
                                onClick={() => setTimeUnit('5D')}
                                bg={timeUnit === '5D' ? '#3861FB' : 'gray.200'}
                                color={timeUnit === '5D' ? 'white' : 'black'}
                            >
                                5D
                            </Button>
                        </ButtonGroup>
                        */}
                    </Flex>
                    <Text fontSize={{ md: '40px', lg: '40px', xl: '40px', '2xl': '40px', '3xl': '40px' }} fontWeight='bold'>{tvl}</Text>
                    <Text fontSize={{ md: '12px', lg: '12px', xl: '12px', '2xl': '12px', '3xl': '12px' }}>{formatDate(currentTime)}</Text>
                </Flex>
            </Flex>
            <CurvedLineChart data={data} options={options} />
        </Box>
    );
};

export default ChartContainer;