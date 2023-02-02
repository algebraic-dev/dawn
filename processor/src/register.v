// Describes a register for pipelining.
module register(clock, clear, hold, in, out);
    input clock, clear, hold;

    input wire [`ARCH_WIDTH-1: 0] in;
    output reg [`ARCH_WIDTH-1: 0] out;

    parameter N = `ARCH_WIDTH;

    always @(posedge clock) begin
        out <= clear ? {N{1'b0}} : (hold ? out : in);
    end
endmodule
