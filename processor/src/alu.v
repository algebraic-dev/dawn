module alu(clock, control, x, y, alt, out);
    input wire clock;
    input wire alt;
    input wire [2:0] control;
    input wire [31:0] x, y;
    output reg [31:0] out;

    always @(posedge clock) begin
        case (control)
            3'b000: out <= alt ? (x - y) : (x + y);
            3'b001: out <= x << y[4:0];
            3'b110: out <= x | y;
            3'b111: out <= x & y;
        endcase
    end
endmodule