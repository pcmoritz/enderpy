use std::{collections::HashMap, path::PathBuf};

use crate::semantic_analyzer::SemanticAnalyzer;
use crate::settings::Settings;
use crate::state::State;
use crate::symbol_table::{SymbolTable, SymbolTableType};

pub struct BuildSource {
    pub path: PathBuf,
    pub module: Option<String>,
    pub source: String,
    // If this source was found by following an import
    pub followed: bool,
}

pub struct BuildManager<'a> {
    errors: Vec<String>,
    sources: Vec<BuildSource>,
    modules: HashMap<String, State<'a>>,
    missing_modules: Vec<String>,
    semantic_analyzer: SemanticAnalyzer,
    options: Settings,
}

impl<'a> BuildManager<'a> {
    pub fn new(
        sources: Vec<BuildSource>,
        semantic_analyzer: SemanticAnalyzer,
        options: Settings,
    ) -> Self {
        if sources.len() > 1 {
            panic!("analyzing more than 1 given input is not supported");
        }

        BuildManager {
            errors: vec![],
            modules: HashMap::new(),
            sources,
            missing_modules: vec![],
            semantic_analyzer,
            options,
        }
    }

    // Entry point to analyze the program
    pub fn build(self) {
        let source = self.sources.last().unwrap();
        let mut modules = HashMap::new();
        modules.insert(
            // TODO: better name
            "mod_name",
            State {
                smybol_table: SymbolTable::new(
                    source.path.to_str().unwrap().to_string(),
                    SymbolTableType::Module,
                    0,
                ),
                path: source.path.clone(),
            },
        );

        // Get module dependencies and create dep graph
        // Process the graph from leaves
    }

    // Adds a source file to the build manager
    fn add_source_file(&mut self, file_path: String) {}

    // Finds imports in the source files and creates an import graph
    fn find_imports(&mut self) {
        // Logic to parse source files and extract imports
        // Populates self.import_graph based on the imports found
    }

    // Performs pre-analysis on the source files
    fn pre_analysis(&mut self) {
        // Logic to perform any pre-analysis required before type checking
    }

    // Performs type checking passes over the code
    fn type_check(&mut self) {
        panic!("not implemented")
    }

    // Performs type checking for a specific file
    fn type_check_file(&mut self, file_path: &str) {
        // Logic to perform type checking for a file
        // Updates self.type_checked_files after successful type checking
    }
}