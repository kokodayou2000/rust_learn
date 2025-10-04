//

#[derive(Debug,PartialEq)]
pub enum TemperatureType {
    FA, // 华氏度
    DE, // 摄氏度
}
pub fn temperature_trans(number: i32,temperature_type: TemperatureType) -> f32 {
    match temperature_type {
        TemperatureType::FA =>  {
            let result = (number as f32) * 9.0 / 5.0 + 32.0;
            println!("摄氏度 = {} 转换成华氏度 = {}",number,result);
            result
        },
        TemperatureType::DE => {
            let result = ((number as f32) -  32.0 ) * 5.0 / 9.0;
            println!("华氏度 = {} 转换成摄氏度 = {}",number,result);
            result
        }
    }
}