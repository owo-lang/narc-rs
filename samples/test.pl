#!/usr/bin/perl

use strict;
use warnings FATAL => 'all';
use Term::ANSIColor;
use v5.10;
$|++;

my $is_release = 0;
say $is_release ? `cargo build --release --bin narc --all-features`
    : `cargo build --bin narc --all-features`;

my $narc = "../target/@{[ $is_release ? 'release' : 'debug' ]}/narc";
my $version = "$narc --version";
say "$version: @{[ `$version` =~ s/[\n\r]//rg ]}, commit @{[ `git rev-parse --short HEAD` ]}";
my @failure = ();
my $success = 0;
my $isCI = defined $ENV{'CI'};

sub ntr {return colored $_[0], 'green';}
sub red {return colored $_[0], 'red';}
sub redy {return colored $_[0], 'bold red';}

foreach my $fixture (map {substr $_, 0, -1}
        split /[ \t\n]+/, `ls -t -d ./*/`) {
    say colored("Fixture $fixture:", 'yellow');
    my $fixtureFlags = -e "$fixture.flags" ? `cat $fixture.flags` : '';
    foreach my $case (split /[ \t\n]+/, `ls -t -G $fixture/*.narc`) {
        my $out = $case =~ s/\.narc/\.out/rg;
        my $flagFile = $case =~ s/\.narc/\.flags/rg;
        my $caseFlags = -e $flagFile ? `cat $flagFile` : '';
        my $flags = "$fixtureFlags $caseFlags" =~ s/[\n|\r]//rg;
        my $cmd = "$narc $flags $case";
        if (!-e $out) {
            print red(" Missing test data for $case, ");
            print colored('create one (y/N)?', 'cyan');
            (readline =~ s/[\n\r]//rg) eq 'y' ? `$cmd > $out 2>&1`
                : say colored('  Leaving it unchanged.', 'bold yellow');
            push @failure, $case;
            next;
        }
        `touch $out`;
        my $diff = `$cmd 2>&1 | diff --strip-trailing-cr - $out`;
        if (length $diff) {
            push @failure, $case;
            say red(" Failed $case:");
            map {say red("  $_")} split /\n/, $diff;
            next if $isCI != 0;
            print colored('  Update the golden value (y/N)? ', 'cyan');
            (readline =~ s/[\n\r]//rg) eq 'y' ? `$cmd > $out 2>&1`
                : say colored(<<"HINT", 'bold yellow');
  Leaving it alone.
  To update the golden value, run `test.pl` in `samples` directly.
  Command: $cmd
HINT
        } else {
            say ntr(" Passed $case");
            $success++;
        }
    }
}

my $failed = scalar @failure;
say 'Result: ', $failed ? redy('FAILED.') : ntr('ok.'),
    ntr(" $success passed,"),
    colored(" $failed failed.", $failed ? 'bold red' : 'white');
if ($failed != 0) {
    my $pretty = join "\n ", @failure;
    say red("Failing tests:\n $pretty");
    die;
}
