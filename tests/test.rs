
extern crate canyon;
use canyon::*;

#[test]
fn test() {
    let mut model = Model::new("alpha");
    {
        let hidden = LayerDesc {
            layr: Layr::Linear,
            actv: Actv::Sigmoid,
            size: 5,
        };

        let batch = LayerDesc {
            layr: Layr::BatchNorm,
            actv: Actv::Sigmoid,
            size: 5,
        };

        let output = LayerDesc {
            layr: Layr::Linear,
            actv: Actv::Linear,
            size: 1,
        };

        model.set_inputs(4);
        model.add_layer(hidden, 1);
        model.add_layer(batch, 1);
        model.add_layer(hidden, 1);
        model.add_output(output);

        model.set_loss(Loss::MAE);
        model.set_optomizer(Opto::Momentum(0.9));

        model.load_csv("iris", |features, inputs, target| {

            inputs.push(features[0].parse::<f32>().unwrap());
            inputs.push(features[1].parse::<f32>().unwrap());
            inputs.push(features[2].parse::<f32>().unwrap());
            inputs.push(features[3].parse::<f32>().unwrap());

            match features[4].as_str() {
                "Setosa" => target.push(0.0),
                "Versicolor" => target.push(1.0),
                "Virginica" => target.push(2.0),
                _ => panic!("Unexpected Iris Name!"),
            }

        });
    }
        
}
