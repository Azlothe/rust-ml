use csv::Reader;
use linfa::Dataset;
use ndarray::{Array, Array1, Array2};
use std::fs::File;

pub fn get_dataset(path: &str) -> Dataset<f32, i32, ndarray::Dim<[usize; 1]>> {
    let mut reader = Reader::from_path(path).unwrap();

    let headers = get_headers(&mut reader);
    let data = get_data(&mut reader);
    let target_index = headers.len() - 1;

    let features = headers[0..target_index].to_vec();
    let records = get_records(&data, target_index);
    let targets = get_targets(&data, target_index);

    return Dataset::new(records, targets).with_feature_names(features);
}

fn get_headers(reader: &mut Reader<File>) -> Vec<String> {
    return reader
        .headers()
        .unwrap()
        .iter()
        .map(|r| r.to_owned())
        .collect();
}

fn get_records(data: &Vec<Vec<f32>>, target_index: usize) -> Array2<f32> {
    let mut records: Vec<f32> = vec![];
    for record in data.iter() {
        records.extend_from_slice(&record[0..target_index]);
    }

    return Array::from(records)
        .into_shape((data.len(), target_index))
        .unwrap();
}

fn get_targets(data: &Vec<Vec<f32>>, target_index: usize) -> Array1<i32> {
    let targets = data
        .iter()
        .map(|record| record[target_index] as i32)
        .collect::<Vec<i32>>();

    return Array::from(targets);
}

fn get_data(reader: &mut Reader<File>) -> Vec<Vec<f32>> {
    return reader
        .records()
        .map(|r| {
            r.unwrap()
                .iter()
                .map(|field| field.parse::<f32>().unwrap())
                .collect::<Vec<f32>>()
        })
        .collect::<Vec<Vec<f32>>>();
}
