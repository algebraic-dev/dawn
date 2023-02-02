`include "src/constants.v"

`include "src/registers.v"
`include "src/register.v"
`include "src/ram.v"
`include "src/alu.v"

module Test;

    reg clock;
    reg clear;
    reg hold;

    reg [`ARCH_WIDTH-1: 0] in;
    wire [`ARCH_WIDTH-1: 0] out;

    register reg1 (
        clock,
        clear,
        hold,
        in,
        out
    );

    initial begin
        clock = 1'b1;

        clear = 1'b0;
        hold = 1'b0;

        in = 32'hFFFF;

        $dumpfile("dump.vcd");

        #1 clock = ~clock;
        #0 in = 32'h00FF;
        #1 clock = ~clock;
        #1 clock = ~clock;
        #1 clock = ~clock;
        #1 clock = ~clock;

        #1 $finish;

        $dumpvars(0, Test);
    end

    always @(clock) $monitor("time=%0t clock = %b; on = %b; in = %b;", $time, clock, out, in);
endmodule


