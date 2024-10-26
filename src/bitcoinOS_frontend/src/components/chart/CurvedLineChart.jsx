import React from 'react';
import { Line } from 'react-chartjs-2';
import { Box } from '@chakra-ui/react';
import {
    Chart as ChartJS,
    CategoryScale,
    LinearScale,
    PointElement,
    LineElement,
    Title,
    Tooltip,
    Legend,
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

const CurvedLineChart = ({ data, options }) => {
    return (
        <Box width="100%">
            <Line data={data} options={options} />
        </Box>
    );
};

export default CurvedLineChart;