// Module declarations for UI components
mod header;  // Header component for application branding
mod navbar;  // Navigation bar component for view switching
mod chart;   // Chart component for data visualization

// Public exports - these components can be used by other modules
pub use header::Header;      // Export Header component
pub use navbar::Navbar;      // Export Navbar component  
pub use chart::ApexChart;    // Export ApexChart component for data visualization
