import * as React from 'react';
import { useListContext } from 'ra-core';
import { Table, TableBody, TableCell, TableHead, TableRow } from '@/components/ui/table';
import { Checkbox } from '@/components/ui/checkbox';

const DynamicTable = () => {
    const { data, isLoading } = useListContext();

    if (isLoading) {
        return <div>Loading...</div>;
    }

    if (!data || data.length === 0) {
        return (
            <TableRow>
                <TableCell colSpan={1} className='h-24 text-center'>
                    No results.
                </TableCell>
            </TableRow>
        );
    }

    const columns = Object.keys(data[0]);

    return (
        <Table>
            <TableHead>
                <TableRow>
                    <TableCell>
                        <Checkbox
                            checked={false}
                            onCheckedChange={() => {}}
                            aria-label='Select all'
                            className='translate-y-[2px]'
                        />
                    </TableCell>
                    {columns.map((column) => (
                        <TableCell key={column}>{column}</TableCell>
                    ))}
                </TableRow>
            </TableHead>
            <TableBody>
                {data.map((record) => (
                    <TableRow key={record.id}>
                        <TableCell>
                            <Checkbox
                                checked={false}
                                onCheckedChange={() => {}}
                                aria-label='Select row'
                                className='translate-y-[2px]'
                            />
                        </TableCell>
                        {columns.map((column) => (
                            <TableCell key={column}>{record[column]}</TableCell>
                        ))}
                    </TableRow>
                ))}
            </TableBody>
        </Table>
    );
};

export default DynamicTable;