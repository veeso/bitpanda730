//! # Stdout
//!
//! This module exposes the stdout paginator for 730

use super::{GainsAndLosses, Module730, Paginate};

/// Stdout paginator
#[derive(Default)]
pub struct Stdout;

impl Paginate for Stdout {
    fn paginate(
        &self,
        module: &Module730,
        gains_and_losses: &GainsAndLosses,
    ) -> anyhow::Result<()> {
        self.print_gains_and_losses(gains_and_losses);
        self.print_quadro_rt(module);
        self.print_quadro_rw(module);
        Ok(())
    }
}

impl Stdout {
    fn print_gains_and_losses(&self, gains_and_losses: &GainsAndLosses) {
        println!("Guadagni e Perdite: ");
        println!();
        for diff in gains_and_losses.iter() {
            if diff.is_gain() {
                println!(
                    "l'asset {} ha registrato un guadagno di € {}, di cui € {} di tasse ({} %)",
                    diff.asset(),
                    diff.value().round_dp(2),
                    diff.tax().round_dp(2),
                    diff.tax_percentage()
                );
            } else {
                println!(
                    "l'asset {} ha registrato una perdita di € {} ({} %)",
                    diff.asset(),
                    diff.value().round_dp(2),
                    diff.tax_percentage()
                );
            }
        }
        println!("--------------------------------------------");
        println!();
    }

    fn print_quadro_rt(&self, module: &Module730) {
        println!("QUADRO RT:");
        println!();
        println!("Sezione I:");
        println!("RT1: € {}", module.quadro_rt.sezione_1.rt1);
        println!("RT2 - Col. 3: € {}", module.quadro_rt.sezione_1.rt2_col3);
        if let Some(col) = module.quadro_rt.sezione_1.rt3_col1 {
            println!("RT3 - Col. 1: € {}", col);
        }
        if let Some(col) = module.quadro_rt.sezione_1.rt3_col2 {
            println!("RT3 - Col. 2: € {}", col);
        }
        println!();
        println!("Sezione II:");
        println!("RT21: € {}", module.quadro_rt.sezione_2.rt21);
        println!("RT22 - Col. 3: € {}", module.quadro_rt.sezione_2.rt22_col3);
        if let Some(col) = module.quadro_rt.sezione_2.rt23_col1 {
            println!("RT23 - Col. 1: € {}", col);
        }
        if let Some(col) = module.quadro_rt.sezione_2.rt23_col2 {
            println!("RT23 - Col. 2: € {}", col);
        }
        println!("--------------------------------------------");
        println!();
    }

    fn print_quadro_rw(&self, module: &Module730) {
        println!("QUADRO RW:");
        println!();
        println!("RW1 - Col.8: € {}", module.quadro_rw.rw1_column8);
        println!("RW1 - Col.11: € {}", module.quadro_rw.rw1_column11);
        println!("--------------------------------------------");
        println!();
    }
}
