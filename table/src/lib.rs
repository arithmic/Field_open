use nalgebra::DMatrix;
use serde::{Deserialize, Serialize};
use traits::traits::Field;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TableOrder {
    ROWS,
    COLUMNS,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Table<F: Field> {
    pub(crate) data: DMatrix<F>,
    order_type: TableOrder,
}

impl<F: Field + 'static> Table<F> {
    pub fn new(data: Vec<Vec<F>>) -> Self {
        let nrows = data.len();
        let ncols = if nrows == 0 { 0 } else { data[0].len() };
        let data_iter = data.into_iter().flatten();
        Self {
            data: DMatrix::from_iterator(nrows, ncols, data_iter),
            order_type: TableOrder::ROWS,
        }
    }
    // Returns the data of tha table
    pub fn data(&self) -> Vec<Vec<F>> {
        self.data
            .row_iter()
            .map(|row| row.iter().cloned().collect())
            .collect()
    }
    // Retuns the bumner of rows of the table
    pub fn num_of_rows(&self) -> usize {
        self.data.nrows()
    }
    // Returns number of columns of the table
    pub fn num_of_columns(&self) -> usize {
        self.data.ncols()
    }

    // Returns the transpose of the table
    pub fn transpose(&self, table_order: TableOrder) -> Self {
        if self.order_type == table_order {
            self.clone()
        } else {
            Self {
                data: self.data.transpose(),
                order_type: table_order,
            }
        }
    }
    // Function to update the table at specific row index.
    pub fn update_row_at(&mut self, idx: usize, row: &[F]) {
        for (j, value) in row.iter().enumerate() {
            self.data[(idx, j)] = *value;
        }
    }
    // Function to update the  row of the table with specific column range
    pub fn update_row_with_column_range(
        &mut self,
        row_idx: usize,
        row: &[F],
        start_index: usize,
        end_index: usize,
    ) {
        // for (j, value) in row.iter().enumerate().take(end_index - start_index).skip(start_index) {
        //     self.data[(row_idx, j)] = *value;
        // }
        let mut count = 0;
        for idx in start_index..end_index {
            self.set_cell(row_idx, idx, row[count]);
            count += 1;
        }
    }

    // Function to update the table with specific column index
    pub fn update_column_at(&mut self, idx: usize, column: &[F]) {
        for (i, value) in column.iter().enumerate() {
            self.data[(i, idx)] = *value;
        }
    }
    // Reurns the value of the column at given index.
    pub fn column_at(&self, index: usize) -> Vec<F> {
        self.data.column(index).iter().cloned().collect()
    }
    // Return the value of the row at given index
    pub fn row_at(&self, index: usize) -> Vec<F> {
        self.data.row(index).iter().cloned().collect()
    }
    // Returns the row values with specific column range
    pub fn row_with_column_range_at(
        &self,
        row_index: usize,
        start_index: usize,
        end_index: usize,
    ) -> Vec<F> {
        self.data
            .row(row_index)
            .iter()
            .skip(start_index)
            .take(end_index - start_index)
            .cloned()
            .collect()
    }
    // Returns the cell value at given row and column index
    pub fn cell_at(&self, row_index: usize, column_index: usize) -> F {
        self.data[(row_index, column_index)]
    }
    // Set the cell at given row and column index.
    pub fn set_cell(&mut self, row_index: usize, column_index: usize, value: F) {
        self.data[(row_index, column_index)] = value;
    }

    // Returns the order type of the table
    pub fn order_type(&self) -> TableOrder {
        self.order_type.clone()
    }
    // Function to read the row values at specific row index.
    pub fn read_row_into(&self, row_idx: usize, row: &mut [F]) {
        for (idx, e) in self.row_at(row_idx).iter().enumerate() {
            row[idx] = *e;
        }
    }
}
