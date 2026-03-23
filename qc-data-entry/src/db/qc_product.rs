use serde::Deserialize;
use serde_rusqlite::from_rows;

// use serde::{Deserialize, Serialize};
// use crate::errors::Result;
use crate::{ProductLine, Range, DB};

// QC standard
#[derive(Deserialize, Debug, Default)]
pub struct QCProductStandard {
    // product_id: u32,
    // ph: Range,
    pub ph: Option<Range>,
    pub sg: Option<Range>,
    pub density: Option<Range>,
    pub string_test: Option<Range>,
    pub viscosity: Option<Range>,
    //
    //
    //
    // product_moniker_name: String,
    // product_name_internal: String,
    // product_lot_id: u32,
    // lot_name: String,
}

// TODO from ProductLine

// impl fmt::Display for QCProduct {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         self.lot_name.fmt(f)
//         // write!(
//         //     f,
//         //     "{}",
//         //     self.lot_name,
//         // )
//     }
// }
impl QCProductStandard {
    pub fn select_product_details(db: &DB, product: &ProductLine) -> Self {
        let qc_test_type_id_ph = 2;
        let qc_test_type_id_sg = 3;
        let qc_test_type_id_density = 4;
        let qc_test_type_id_string_test = 5;
        let qc_test_type_id_viscosity = 6;
        //
        //
        //
        Self {
            ph: Range::select_product_lot_product(db, &product.product_id, &qc_test_type_id_ph),
            sg: Range::select_product_lot_product(db, &product.product_id, &qc_test_type_id_sg),
            density: Range::select_product_lot_product(
                db,
                &product.product_id,
                &qc_test_type_id_density,
            ),
            string_test: Range::select_product_lot_product(
                db,
                &product.product_id,
                &qc_test_type_id_string_test,
            ),
            viscosity: Range::select_product_lot_product(
                db,
                &product.product_id,
                &qc_test_type_id_viscosity,
            ),
        }
    }
}
// impl QCProduct {
//         pub fn select_product_details(db: &DB, product: &ProductLine) -> Vec<Self> {
//         let mut statement = db
//             .prepare(
//                 "
//                 with
// 	val
// 	(
// 		product_id,
// 		ph_method,
//         ph_measure,
//         ph_publish,
//         ph_min,
//         ph_target,
//         ph_max,

//         specific_gravity_method,
//         specific_gravity_measure,
//         specific_gravity_publish,
//         specific_gravity_min,
//         specific_gravity_target,
//         specific_gravity_max,

//         density_method,
//         density_measure,
//         density_publish,
//         density_min,
//         density_target,
//         density_max,

//         string_test_method,
//         string_test_measure,
//         string_test_publish,
//         string_test_min,
//         string_test_target,
//         string_test_max,

//         viscosity_method,
//         viscosity_measure,
//         viscosity_publish,
//         viscosity_min,
//         viscosity_target,
//         viscosity_max
// 	)
// 	as (
// 	select product_id,
// 		max(case when qc_test_type_id = 2 then qc_test_method_name end) as ph_method,
// 		max(case when qc_test_type_id = 2 then val_measure end) as ph_measure,
// 		max(case when qc_test_type_id = 2 then val_publish end) as ph_publish,
// 		max(case when qc_test_type_id = 2 then val_min end) as ph_min,
// 		max(case when qc_test_type_id = 2 then val_target end) as ph_target,
// 		max(case when qc_test_type_id = 2 then val_max end) as ph_max,

// 		max(case when qc_test_type_id = 3 then qc_test_method_name end) as specific_gravity_method,
// 		max(case when qc_test_type_id = 3 then val_measure end) as specific_gravity_measure,
// 		max(case when qc_test_type_id = 3 then val_publish end) as specific_gravity_publish,
// 		max(case when qc_test_type_id = 3 then val_min end) as specific_gravity_min,
// 		max(case when qc_test_type_id = 3 then val_target end) as specific_gravity_target,
// 		max(case when qc_test_type_id = 3 then val_max end) as specific_gravity_max,

// 		max(case when qc_test_type_id = 4 then qc_test_method_name end) as density_method,
// 		max(case when qc_test_type_id = 4 then val_measure end) as density_measure,
// 		max(case when qc_test_type_id = 4 then val_publish end) as density_publish,
// 		max(case when qc_test_type_id = 4 then val_min end) as density_min,
// 		max(case when qc_test_type_id = 4 then val_target end) as density_target,
// 		max(case when qc_test_type_id = 4 then val_max end) as density_max,

// 		max(case when qc_test_type_id = 5 then qc_test_method_name end) as string_test_method,
// 		max(case when qc_test_type_id = 5 then val_measure end) as string_test_measure,
// 		max(case when qc_test_type_id = 5 then val_publish end) as string_test_publish,
// 		max(case when qc_test_type_id = 5 then val_min end) as string_test_min,
// 		max(case when qc_test_type_id = 5 then val_target end) as string_test_target,
// 		max(case when qc_test_type_id = 5 then val_max end) as string_test_max,

// 		max(case when qc_test_type_id = 6 then qc_test_method_name end) as viscosity_method,
// 		max(case when qc_test_type_id = 6 then val_measure end) as viscosity_measure,
// 		max(case when qc_test_type_id = 6 then val_publish end) as viscosity_publish,
// 		max(case when qc_test_type_id = 6 then val_min end) as viscosity_min,
// 		max(case when qc_test_type_id = 6 then val_target end) as viscosity_target,
// 		max(case when qc_test_type_id = 6 then val_max end) as viscosity_max

// 	from bs.product_ranges_measured
// 	left join bs.qc_test_methods using (qc_test_method_id)
// 	where product_id = ?1
// 	)

// 	select

// 	product_type_id,
// 	container_type_id,
// 	product_appearance_text,

// 	ph_method,
//     ph_measure,
//     ph_publish,
//     ph_min,
//     ph_target,
//     ph_max,

//     specific_gravity_method,
//     specific_gravity_measure,
//     specific_gravity_publish,
//     specific_gravity_min,
//     specific_gravity_target,
//     specific_gravity_max,

//     density_method,
//     density_measure,
//     density_publish,
//     density_min,
//     density_target,
//     density_max,

//     string_test_method,
//     string_test_measure,
//     string_test_publish,
//     string_test_min,
//     string_test_target,
//     string_test_max,

//     viscosity_method,
//     viscosity_measure,
//     viscosity_publish,
//     viscosity_min,
//     viscosity_target,
//     viscosity_max

// 	from bs.product_attributes
// 	left join val using (product_id)
// 	left join bs.product_appearance using (product_appearance_id)
// 	join bs.product_types using (product_type_id)

// 	where product_id = ?1
// ",
//             )
//             .unwrap();
//         // from_rows::<Self>(statement.query([product.product_id]).unwrap())
//         from_rows::<Self>(statement.query([product.product_id]).unwrap())
//             // .map(|x| x.unwrap())
//             .map(|x| x.map_err(|x| println!("error! {x:?}")).unwrap())
//             .collect()
//     }
// }

/*


// bs.product_attributes
    // bs.product_ranges_measured
    BIG_SEL_RANGES_QC := `
    ph_method,
    ph_measure,
    ph_publish,
    ph_min,
    ph_target,
    ph_max,

    specific_gravity_method,
    specific_gravity_measure,
    specific_gravity_publish,
    specific_gravity_min,
    specific_gravity_target,
    specific_gravity_max,

    density_method,
    density_measure,
    density_publish,
    density_min,
    density_target,
    density_max,

    string_test_method,
    string_test_measure,
    string_test_publish,
    string_test_min,
    string_test_target,
    string_test_max,

    viscosity_method,
    viscosity_measure,
    viscosity_publish,
    viscosity_min,
    viscosity_target,
    viscosity_max
    `

    DB_Select_product_details = PrepareOrElse(db, `
    with
    val
    (
        product_id,
    `+BIG_SEL_RANGES_QC+`
    )
    as (
    select product_id,
        max(case when qc_test_type_id = 2 then qc_test_method_name end) as ph_method,
        max(case when qc_test_type_id = 2 then val_measure end) as ph_measure,
        max(case when qc_test_type_id = 2 then val_publish end) as ph_publish,
        max(case when qc_test_type_id = 2 then val_min end) as ph_min,
        max(case when qc_test_type_id = 2 then val_target end) as ph_target,
        max(case when qc_test_type_id = 2 then val_max end) as ph_max,

        max(case when qc_test_type_id = 3 then qc_test_method_name end) as specific_gravity_method,
        max(case when qc_test_type_id = 3 then val_measure end) as specific_gravity_measure,
        max(case when qc_test_type_id = 3 then val_publish end) as specific_gravity_publish,
        max(case when qc_test_type_id = 3 then val_min end) as specific_gravity_min,
        max(case when qc_test_type_id = 3 then val_target end) as specific_gravity_target,
        max(case when qc_test_type_id = 3 then val_max end) as specific_gravity_max,

        max(case when qc_test_type_id = 4 then qc_test_method_name end) as density_method,
        max(case when qc_test_type_id = 4 then val_measure end) as density_measure,
        max(case when qc_test_type_id = 4 then val_publish end) as density_publish,
        max(case when qc_test_type_id = 4 then val_min end) as density_min,
        max(case when qc_test_type_id = 4 then val_target end) as density_target,
        max(case when qc_test_type_id = 4 then val_max end) as density_max,

        max(case when qc_test_type_id = 5 then qc_test_method_name end) as string_test_method,
        max(case when qc_test_type_id = 5 then val_measure end) as string_test_measure,
        max(case when qc_test_type_id = 5 then val_publish end) as string_test_publish,
        max(case when qc_test_type_id = 5 then val_min end) as string_test_min,
        max(case when qc_test_type_id = 5 then val_target end) as string_test_target,
        max(case when qc_test_type_id = 5 then val_max end) as string_test_max,

        max(case when qc_test_type_id = 6 then qc_test_method_name end) as viscosity_method,
        max(case when qc_test_type_id = 6 then val_measure end) as viscosity_measure,
        max(case when qc_test_type_id = 6 then val_publish end) as viscosity_publish,
        max(case when qc_test_type_id = 6 then val_min end) as viscosity_min,
        max(case when qc_test_type_id = 6 then val_target end) as viscosity_target,
        max(case when qc_test_type_id = 6 then val_max end) as viscosity_max

    from bs.product_ranges_measured
    left join bs.qc_test_methods using (qc_test_method_id)
    where product_id = ?1
    )

    select

    product_type_id,
    container_type_id,
    product_appearance_text,
    `+BIG_SEL_RANGES_QC+`

    from bs.product_attributes
    left join val using (product_id)
    left join bs.product_appearance using (product_appearance_id)
    join bs.product_types using (product_type_id)

    where product_id = ?1
    `)
    // group by product_id

    BIG_RANGES_QC := `
    val_measure,
    val_publish,
    val_min,
    val_target,
    val_max`
    BIG_NAME_RANGES_QC := `product_id, qc_test_type_name,
    qc_test_method_name,
    ` + BIG_RANGES_QC

    BIG_ID_RANGES_QC := `product_id,
    qc_test_type_id,
    qc_test_method_id,
    ` + BIG_RANGES_QC

    BIG_EXCLUDED_QC := `
qc_test_method_id=excluded.qc_test_method_id,
val_measure=excluded.val_measure,
val_publish=excluded.val_publish,
val_min=excluded.val_min,
val_target=excluded.val_target,
val_max=excluded.val_max`
    DB_Upsert_product_details = PrepareOrElse(db, `
    with
    val
    (
        `+BIG_NAME_RANGES_QC+`
    )
    as (
        values (
            ?,?,
            ?,?,?,?,?,?)
    ),

    sel as (select
        `+BIG_ID_RANGES_QC+`
    from val
    join bs.qc_test_types using (qc_test_type_name)
    left join bs.qc_test_methods using (qc_test_method_name)
)

    insert into bs.product_ranges_measured
    (
        `+BIG_ID_RANGES_QC+`
    )
    select
    `+BIG_ID_RANGES_QC+`
    from sel
    where true
    on conflict(product_id, qc_test_type_id) do update set
    `+BIG_EXCLUDED_QC+`
    returning range_id
    `)

    product_type_id := `product_id,
    product_type_id,
    product_appearance_id`
    DB_Upsert_product_type = PrepareOrElse(db, `
    with
    val
    (
        product_id,
        product_type_id,
        product_appearance_text
    )
    as (
        values (
            ?,?,?
        )
    ),
    sel as (
        select
        `+product_type_id+`
        from val
        left join bs.product_appearance using (product_appearance_text)
    )
    insert into bs.product_attributes	(
        `+product_type_id+`
    )
    select
        `+product_type_id+`
    from sel
    where true
    on conflict(product_id) do update set
    product_type_id=excluded.product_type_id,
    product_appearance_id=excluded.product_appearance_id

    returning product_attribute_id
    `)


    type QCProduct struct {
        BaseProduct
        Appearance     ProductAppearance
        Product_type   Discrete
        Container_type ProductContainerType // bs.container_types
        PH             datatypes.Range
        SG             datatypes.Range
        Density        datatypes.Range
        String_test    datatypes.Range
        Viscosity      datatypes.Range
        UpdateFN       func(*QCProduct)
}

type BaseProduct struct {
    Product_name             string `json:"product_name"`
    Lot_number               string `json:"lot_number"`
    Sample_point             string
    Tester                   nullable.NullString `json:"Tester"`
    Visual                   bool
    Product_id               int64
    Lot_id                   int64
    Product_Lot_id           int64
    Product_name_customer_id nullable.NullInt64
    Product_name_customer    string `json:"customer_product_name"`
    Blend                    *blender.ProductBlend
    Valid                    bool
}

let row1 = Example { id: 1, name: "first name".into() };
connection.execute("INSERT INTO example (id, name) VALUES (:id, :name)", to_params_named(&row1).unwrap().to_slice().as_slice()).unwrap();
// and limiting the set of fields that are to be serialized*/
