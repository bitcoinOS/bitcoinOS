import React from 'react';
import { Doughnut } from 'react-chartjs-2';
import { Box } from '@chakra-ui/react';
import {
    Chart as ChartJS,
    ArcElement,
    Tooltip,
    Legend,
} from 'chart.js';

ChartJS.register(
    ArcElement,
    Tooltip,
    Legend
);

const AssetChart = ({ data, options }) => {
    return (
        <Box width="100%" height="400px">
            <Doughnut data={data} options={options} />
        </Box>
    );
};

export default AssetChart;