module ram(clock, i_addr, i_data, d_addr, d_data, read_data);

    input wire clock, rd, wr;

    input wire [`ARCH_WIDTH-1:0] i_addr, d_addr;
    output reg [`ARCH_WIDTH-1:0] i_data, d_data;

    // 32 bits memory with 1024 entries
    reg [`ARCH_WIDTH-1:0] data [0:1024];

    always @(posedge clock) begin
        if (wr) begin
            data[addr] <= write_data;
        end
    end

    // I think that it avoids one cycle when it changes
    assign read_data = rd ? write_data : data[addr];
endmodule