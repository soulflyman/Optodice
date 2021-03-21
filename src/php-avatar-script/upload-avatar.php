<?php
    error_reporting(E_ALL);   
    set_error_handler(function($errno, $errstr, $errfile, $errline) {
        // error was suppressed with the @-operator
        if (0 === error_reporting()) {
            return false;
        }
        
        throw new ErrorException($errstr, 0, $errno, $errfile, $errline);
    });

    function write_log($msg, $debug)
    {
        if (!$debug) return;

        file_put_contents(__FILE__.".log", $msg.PHP_EOL , FILE_APPEND | LOCK_EX);
    }

    $debug = false;
    if (array_key_exists("debug", $_POST)) {
        $positive_debug_values = [1, "1", "on", "ON", "On", "true", "True", "TRUE", "enable", "Enable", "ENABLE"];
        $debug = in_array($_POST["debug"], $positive_debug_values);
    }

    try {
        write_log("---", $debug);
        
        $target_file = $_POST["hero_id"]."_".$_POST["checksum"].".png";
        if ( file_exists("./".$target_file) ) {
            write_log("Warning: Target file exists: ./".$target_file, $debug);
            return;
        }
        
        $mime_types = ["data:image/jpeg;base64,", "data:image/jpg;base64,", "data:image/png;base64,"];
        
        $code_base64 = $_POST["image"];    
        foreach($mime_types as $mime_type) {
            $code_base64 = str_replace($mime_type,'',$code_base64);
        }    
        $code_binary = base64_decode($code_base64);
        $image= imagecreatefromstring($code_binary);
        imagepng($image, "./" . $target_file);
    } catch (ErrorException $e) {
        write_log($e->getMessage(), $debug);
        write_log(var_export($_POST, true), $debug);        
    }
