use calamine::{Writer, Xlsx};

pub fn create_workbook() -> Xlsx<_> {
    let mut workbook: Xlsx<_> = Xlsx::new();
    let mut sheet = workbook.create_sheet("Данные");
    workbook
}

pub fn fill_data(workbook: Xlsx<_>, &sheet: Any, json: Json) {
    workbook.write(&mut sheet)
}
