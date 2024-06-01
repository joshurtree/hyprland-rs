/// Demonstrates usage of various dispatch calls
///
/// Usage: cargo run --example dispatch <hyprland args>? <program_name>? <program_args>?
/// Example: cargo run --example dispatch [workspace 2] kitty 

use hyprland::dispatch;
use hyprland::dispatch::{Corner, Dispatch, DispatchType, FullscreenType, WorkspaceIdentifierWithSpecial};
use hyprland::dispatch::DispatchType::*;

fn describe(desc: &str) {
    std::thread::sleep(std::time::Duration::from_secs(2)); 
    println!("{}", desc); 
}

fn main() -> hyprland::Result<()>{
    let args = std::env::args().skip(1).collect::<Vec<_>>().join(" ");
    let program = if args.len() == 0 { "kitty" } else { &args };
    println!("Executing {program}");
    dispatch!(Exec, program)?;

    describe("Moving cursor to top left");
	dispatch!(MoveCursorToCorner, Corner::TopLeft)?;

    describe("Moving cursor to top right");
	dispatch!(MoveCursorToCorner, Corner::TopRight)?;

    describe("Moving cursor to bottom right");
	dispatch!(MoveCursorToCorner, Corner::BottomRight)?;

    describe("Moving cursor to bottom left");
	dispatch!(MoveCursorToCorner, Corner::BottomLeft)?;

    describe("Moving window to next workspace");
	dispatch!(MoveToWorkspace, WorkspaceIdentifierWithSpecial::Relative(1), None)?;

    describe("Moving window to previous workspace");
	dispatch!(MoveToWorkspace, WorkspaceIdentifierWithSpecial::Relative(-1), None)?;

    describe("Toggling fullscreen");
	dispatch!(ToggleFullscreen, FullscreenType::Maximize)?;
    describe("Reverting fullscreen");
	Dispatch::call(ToggleFullscreen(FullscreenType::Maximize))?;

    describe("Toggling floating window");
	dispatch!(ToggleFloating, None)?;
    describe("Reverting floating window");
	Dispatch::call(ToggleFloating(None))?;

    describe("Toggling split layout");
	Dispatch::call(ToggleSplit)?;
    describe("Reverting split layout");
	Dispatch::call(ToggleSplit)?;

    describe("Toggling opaque");
	Dispatch::call(ToggleOpaque)?;
    describe("Reverting opaque");
	Dispatch::call(ToggleOpaque)?;

    describe("Closing window");
    Dispatch::call(KillActiveWindow)?;

    Ok(())
}   