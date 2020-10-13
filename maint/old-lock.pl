#!perl
use strict;
use warnings;

# This is very crude.
# 
# 1. Read ./Cargo.lock trying to extract crate name/version/checksum/source details from 'new' format
# 2. Read Cargo.lock a second time, writing it to Cargo.lock.new streamed, but
#    a. strip checksum = from [[ package ]] entries on the way through
#    b. rewrite dependency = [ sections using data collected on the first pass
# 3. Then, generate a [metadata] section from the data obtained in the first pass and emit to the new file.
#
# Its up to you to copy the new file back into place.

my $states = parse_meta("Cargo.lock");
rewrite_file("Cargo.lock","Cargo.lock.new", $states);
1;

sub fix_dep {
    my ( $depstring, $states ) = @_;
    if ( $depstring =~ /^([^ ]+)$/ ) {

        # single map
        my $package = $1;
        if ( not exists $states->{$package} ) {
            die "$package not found";
        }
        my ( $first, @garbage ) = values %{ $states->{$package} };
        if (@garbage) {
            die "Too many versions for $package";
        }
        return
            $package . " "
          . $first->{version} . " ("
          . $first->{source} . ")";
    }
    if ( $depstring =~ /^([^ ]+)[ ]+([^ ]+)$/ ) {
        my ( $package, $version ) = ( $1, $2 );
        if ( not exists $states->{$package} ) {
            die "$package not found";
        }
        if ( not exists $states->{$package}->{$version} ) {
            die "$package v$version not found";
        }
        return
            $package . " "
          . $version . " ("
          . $states->{$package}->{$version}->{source} . ")";
    }
    die "Unhandled depstring $depstring";
}

sub parse_meta {
    my ($filename) = @_;
    open my $fh, "<", $filename or die "Can't read $filename";

    my $states = {};

  lines: while ( my $line = <$fh> ) {
        next unless $line =~ /^\[\[package\]\]/;
        my $record;

      fields: while ( my $line = <$fh> ) {
            last fields if $line =~ /^\s*$/;
            if ( $line =~ /^([^ ]+) = "([^"]+)"/ ) {
                my ( $field, $value ) = ( $1, $2 );
                $record->{$field} = $value;
            }
        }
        if ( not exists $record->{name} ) {
            die "Didn't parse record name";
        }
        if ( not exists $record->{version} ) {
            die "Didn't parse record version for $record->{name}";
        }
        if ( not exists $record->{source} ) {
            warn
"Didn't parse record source for $record->{name} $record->{version}";
        }
        if ( not exists $record->{checksum} ) {
            warn
"Didn't parse record checksum for $record->{name} $record->{version}";
        }
        $states->{ $record->{name} }->{ $record->{version} } = $record;
    }
    return $states;
}

sub rewrite_file {
    my ( $src, $dest, $states ) = @_;
    open my $fh,  "<", $src  or die "Can't read $src";
    open my $wfh, ">", $dest or die "Cant write $dest";

  lines: while ( my $line = <$fh> ) {
        if ( $line !~ /^\[\[package\]\]/ ) {
            $wfh->print($line);
            next lines;
        }
        $wfh->print($line);
      fields: while ( my $line = <$fh> ) {
            if ( $line =~ /^\s*$/ ) {
                $wfh->print($line);
                last fields;
            }

            if ( $line =~ /^([^ ]+) = "/ ) {
                if ( "$1" ne "checksum" ) {
                    $wfh->print($line);
                    next fields;
                }
            }
            if ( $line =~ /^dependencies = \[/ ) {
                $wfh->print($line);
                dep: while ( my $dep = <$fh> ) {
                    if ( $dep =~ /^\]/ ) {
                        $wfh->print($dep);
                        next fields;
                    }
                    if ( $dep =~ /^(\s+")([^"]+)(".+\z)/ms ) {
                        my ( $lpad, $content, $rpad ) = ( $1, $2, $3 );
                        $wfh->print(
                            $lpad . fix_dep( $content, $states ) . $rpad );
                        next dep;
                    }
                    die "Unhandled $dep";
                }
            }
        }
    }
    # write a metadata section
    $wfh->print("\n[metadata]\n");
    for my $crate ( sort keys %{$states} ) {
      for my $version ( sort keys %{$states->{$crate}} ) {
        next unless exists $states->{$crate}->{$version}->{checksum};
        $wfh->printf(qq["checksum %s %s (%s)" = "%s"\n],
          $crate,
          $version,
          $states->{$crate}->{$version}->{source},
          $states->{$crate}->{$version}->{checksum});
      }
    }
}

1;
