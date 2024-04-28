# Table 
The respository has the following structure :
 * Table crate <br>

Table crate includes the **Table** struct which helps us to write data in a table form .i.e in the form of rows and columns along with **TableOrder** enum which specifies the rows and columns of the table.

##### Structure of struct and enum in the crate :
```
pub enum TableOrder {
    ROWS,
    COLUMNS,
}

pub struct Table<F: Field> {
    pub data: Vec<Vec<F>>,
    order_type: TableOrder,
}
```

### How to use table crate in your project:
* Bring the table crate into your project by using : <br>
table = { git = "ssh://git@github.com/arithmic/table.git", branch = "main" }

### Dependencies :
* field traits repository [https://github.com/arithmic/field_trait.git].
