use flux::plugin::FluxPlugin;

#[derive(Default)]
pub struct ExamplePlugin {
    name: String,
    config: Option<String>,
}

impl FluxPlugin for ExamplePlugin {
    fn name(&self) -> &str {
        "example"
    }

    fn version(&self) -> &str {
        "1.0.0"
    }

    fn description(&self) -> &str {
        "A simple example plugin that performs basic arithmetic operations"
    }

    fn commands(&self) -> Vec<(&str, &str)> {
        vec![
            ("add", "Add two numbers: add <num1> <num2>"),
            ("sub", "Subtract two numbers: sub <num1> <num2>"),
            ("mul", "Multiply two numbers: mul <num1> <num2>"),
            ("div", "Divide two numbers: div <num1> <num2>"),
            ("help", "Show this help message"),
        ]
    }

    fn init(&mut self) -> Result<(), String> {
        self.name = "Example Plugin".to_string();
        println!("Example plugin v{} initialized!", self.version());
        Ok(())
    }

    fn execute(&self, args: &[String]) -> Result<(), String> {
        match args.get(0).map(String::as_str) {
            Some("add") => self.handle_math(args, |a, b| a + b, "+"),
            Some("sub") => self.handle_math(args, |a, b| a - b, "-"),
            Some("mul") => self.handle_math(args, |a, b| a * b, "*"),
            Some("div") => self.handle_math(args, |a, b| {
                if b == 0 {
                    return Err("Division by zero!".to_string());
                }
                Ok(a / b)
            }, "/"),
            Some("help") => {
                println!("{}", self.help());
                Ok(())
            }
            _ => Err(format!("Unknown command. Use '{} help' for usage", self.name()))
        }
    }

    fn cleanup(&mut self) -> Result<(), String> {
        println!("Example plugin cleaned up!");
        Ok(())
    }

    fn configure(&mut self, config: &str) -> Result<(), String> {
        self.config = Some(config.to_string());
        println!("Plugin configured with: {}", config);
        Ok(())
    }
}

impl ExamplePlugin {
    fn handle_math<F>(&self, args: &[String], op: F, symbol: &str) -> Result<(), String>
    where
        F: Fn(i64, i64) -> Result<i64, String>
    {
        if args.len() != 3 {
            return Err(format!("Usage: {} {} <num1> <num2>", self.name(), args[0]));
        }
        
        let num1: i64 = args[1].parse()
            .map_err(|e: std::num::ParseIntError| e.to_string())?;
        let num2: i64 = args[2].parse()
            .map_err(|e: std::num::ParseIntError| e.to_string())?;
        
        match op(num1, num2) {
            Ok(result) => {
                println!("{} {} {} = {}", num1, symbol, num2, result);
                Ok(())
            }
            Err(e) => Err(e)
        }
    }
}

#[no_mangle]
pub fn create_plugin() -> Box<dyn FluxPlugin> {
    Box::new(ExamplePlugin::default())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let plugin = ExamplePlugin::default();
        let args = vec!["add".to_string(), "5".to_string(), "3".to_string()];
        assert!(plugin.execute(&args).is_ok());
    }

    #[test]
    fn test_div_by_zero() {
        let plugin = ExamplePlugin::default();
        let args = vec!["div".to_string(), "5".to_string(), "0".to_string()];
        assert!(plugin.execute(&args).is_err());
    }
} 