# Regression test for issue #9422: [load_static_extension db csv] must
# register the csv virtual table module, as upstream testfixture does.
#
# Run via:
#   make -C bindings/tcl build && tclsh bindings/tcl/test_csv_module.tcl

set pass 0
set fail 0

proc check {label want script} {
    if {[catch {uplevel 1 $script} got]} {
        set got "ERROR: $got"
    }
    if {$got eq $want} {
        puts "PASS  $label"
        incr ::pass
    } else {
        puts "FAIL  $label"
        puts "      want: $want"
        puts "      got:  $got"
        incr ::fail
    }
}

set here [file dirname [file normalize [info script]]]
set lib  [file join $here libturso_tcl.so]
if {[catch {load $lib Tursotcl} err]} {
    puts "ERROR: failed to load $lib: $err"
    exit 1
}

set dir [file join [pwd] [format "csv-9422-%d" [pid]]]
file mkdir $dir
set csvfile [file join $dir csv.data]
set fd [open $csvfile w]
puts $fd "a,b"
puts -nonewline $fd "abcd,efgh"
close $fd

for {set i 0} {$i < 3} {incr i} {
    sqlite3 db :memory:
    load_static_extension db csv
    check "csv module works on connection $i" {abcd efgh} {
        db eval "CREATE VIRTUAL TABLE abc USING csv(filename='$csvfile', header=true)"
        db eval {SELECT * FROM abc}
    }
    db close
}

sqlite3 db :memory:
load_static_extension db csv
check "bare header parameter means header=true" {a b} {
    db eval {CREATE VIRTUAL TABLE t1 USING csv(data='a,b
1,2', header)}
    db eval {SELECT name FROM pragma_table_info('t1')}
}
check "unknown extension names are still accepted" {} {
    load_static_extension db no_such_extension
}
db close

file delete -force $dir

puts ""
puts "$pass passed, $fail failed"
if {$fail > 0} { exit 1 }
