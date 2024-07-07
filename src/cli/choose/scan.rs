use crate::cli::context::Context;

pub fn scan_rolls(ctx: &mut dyn Context) -> Result<usize, String> {
    let mut rolls = 0;

    while rolls == 0 {
        rolls = match ctx.scan_usize("How many times do you want to roll the dice?".to_string()) {
            Ok(pass) => pass,
            Err(_) => {
                match ctx.print("You must enter a positive number".to_string()) {
                    Ok(_) => {
                        continue;
                    },
                    Err(error) => return Err(error)
                };
            }
        };

        if rolls == 0 {
            match ctx.print("Roll the dice at least once".to_string()) {
                Ok(_) => {},
                Err(error) => return Err(error)
            };
        }
    }

    return Ok(rolls);
}

pub fn scan_variants(ctx: &mut dyn Context) -> Result<Vec<String>, String> {
    let mut count: usize = 0;

    while count == 0 {
        count = match ctx.scan_usize("How many variants do you want to enter?".to_string()) {
            Ok(count) => count,
            Err(_) => {
                match ctx.print("You must enter a positive number".to_string()) {
                    Ok(_) => {
                        continue;
                    },
                    Err(error) => return Err(error)
                };
            }
        };

        if count == 0 {
            match ctx.print("Enter at least one variant".to_string()) {
                Ok(_) => {},
                Err(error) => return Err(error)
            };
        }
    }

    let mut variants: Vec<String> = Vec::with_capacity(count as usize);

    for index in 0..count {
        let mut variant = String::new();

        while variant.len() == 0 {
            variant = match ctx.scan_string(format!("Enter the {} variant", index + 1)) {
                Ok(variant) => variant,
                Err(_) => {
                    match ctx.print("You must enter a variant".to_string()) {
                        Ok(_) => {
                            continue;
                        },
                        Err(error) => return Err(error)
                    };
                }
            };
        }

        variants.push(variant);
    }

    return Ok(variants);
}
