extern crate clang_sys;

use std::ffi::CString;

fn main()
{
    let header_file = CString::new("/home/hel/Documents/CargoWorkspace/clang_test/test_files/test.cpp").expect("CString::new failed");

    unsafe
    {
        let index = clang_sys::clang_createIndex(0, 0);

        let translation_unit = clang_sys::clang_parseTranslationUnit(
            index,
            header_file.as_ptr(),
            std::ptr::null(),
            0,
            core::ptr::null_mut(),
            0,
            0,
        );

        if translation_unit.is_null()
        {
            panic!("Failed to parse the translation unit");
        }

        // Visit each cursor in the translation unit
        clang_sys::clang_visitChildren(
            clang_sys::clang_getTranslationUnitCursor(translation_unit),
            visit_cursor,
            std::ptr::null_mut(),
        );

        // Dispose of the translation unit and index
        clang_sys::clang_disposeTranslationUnit(translation_unit);
        clang_sys::clang_disposeIndex(index);
    }
}

extern "C" fn visit_cursor(
    cursor: clang_sys::CXCursor,
    _: clang_sys::CXCursor,
    data: clang_sys::CXClientData,
) -> clang_sys::CXChildVisitResult
{
    unsafe
    {
        let cursor_kind = clang_sys::clang_getCursorKind(cursor);

        match cursor_kind {
            clang_sys::CXCursor_FunctionDecl => {
                // This cursor represents a function declaration

                // Get the function name
                let function_name = clang_sys::clang_getCursorSpelling(cursor);
                //let function_name_str = CString::from_raw(function_name).to_string_lossy();

                println!("Function: {:?}", function_name);

                // Visit function arguments
                //clang_sys::clang_visitChildren(cursor, Some(visit_argument), data);
            }
            _ => {}
        }

        clang_sys::CXChildVisit_Continue
    }
}

/*
extern crate clang_sys;

use std::ffi::CString;

fn main() {
    unsafe {
        // Set the path to the header file you want to analyze
        let header_file = CString::new("path/to/your/header.h").expect("CString::new failed");

        // Create an index
        let index = clang_sys::clang_createIndex(0, 0);

        // Parse the header file
        let translation_unit = clang_sys::clang_parseTranslationUnit(
            index,
            header_file.as_ptr(),
            std::ptr::null(),
            0,
            std::ptr::null(),
            0,
            0,
        );

        if translation_unit.is_null() {
            panic!("Failed to parse the translation unit");
        }

        // Visit each cursor in the translation unit
        clang_sys::clang_visitChildren(
            clang_sys::clang_getTranslationUnitCursor(translation_unit),
            Some(visit_cursor),
            std::ptr::null_mut(),
        );

        // Dispose of the translation unit and index
        clang_sys::clang_disposeTranslationUnit(translation_unit);
        clang_sys::clang_disposeIndex(index);
    }
}

unsafe extern "C" fn visit_cursor(
    cursor: clang_sys::CXCursor,
    _: clang_sys::CXCursor,
    data: clang_sys::CXClientData,
) -> clang_sys::CXChildVisitResult {
    let cursor_kind = clang_sys::clang_getCursorKind(cursor);

    match cursor_kind {
        clang_sys::CXCursor_FunctionDecl => {
            // This cursor represents a function declaration

            // Get the function name
            let function_name = clang_sys::clang_getCursorSpelling(cursor);
            let function_name_str = CString::from_raw(function_name).to_string_lossy();

            println!("Function: {}", function_name_str);

            // Visit function arguments
            clang_sys::clang_visitChildren(cursor, Some(visit_argument), data);
        }
        _ => {}
    }

    clang_sys::CXChildVisit_Continue
}

unsafe extern "C" fn visit_argument(
    cursor: clang_sys::CXCursor,
    _: clang_sys::CXCursor,
    _: clang_sys::CXClientData,
) -> clang_sys::CXChildVisitResult {
    // This cursor represents a function argument

    // Get the argument name
    let argument_name = clang_sys::clang_getCursorSpelling(cursor);
    let argument_name_str = CString::from_raw(argument_name).to_string_lossy();

    println!("  Argument: {}", argument_name_str);

    clang_sys::CXChildVisit_Continue
}
*/
