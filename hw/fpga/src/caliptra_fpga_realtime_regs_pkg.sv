// Generated by PeakRDL-regblock - A free and open-source SystemVerilog generator
//  https://github.com/SystemRDL/PeakRDL-regblock

package caliptra_fpga_realtime_regs_pkg;

    localparam CALIPTRA_FPGA_REALTIME_REGS_DATA_WIDTH = 32;
    localparam CALIPTRA_FPGA_REALTIME_REGS_MIN_ADDR_WIDTH = 13;

    typedef struct {
        logic [31:0] next;
    } interface_regs__generic_output_wires__value__in_t;

    typedef struct {
        interface_regs__generic_output_wires__value__in_t value;
    } interface_regs__generic_output_wires__in_t;

    typedef struct {
        logic next;
    } interface_regs__status__cptra_error_fatal__in_t;

    typedef struct {
        logic next;
    } interface_regs__status__cptra_error_non_fatal__in_t;

    typedef struct {
        logic next;
    } interface_regs__status__ready_for_fuses__in_t;

    typedef struct {
        logic next;
    } interface_regs__status__ready_for_fw_push__in_t;

    typedef struct {
        logic next;
    } interface_regs__status__ready_for_runtime__in_t;

    typedef struct {
        logic next;
    } interface_regs__status__mailbox_data_avail__in_t;

    typedef struct {
        logic next;
    } interface_regs__status__mailbox_flow_done__in_t;

    typedef struct {
        interface_regs__status__cptra_error_fatal__in_t cptra_error_fatal;
        interface_regs__status__cptra_error_non_fatal__in_t cptra_error_non_fatal;
        interface_regs__status__ready_for_fuses__in_t ready_for_fuses;
        interface_regs__status__ready_for_fw_push__in_t ready_for_fw_push;
        interface_regs__status__ready_for_runtime__in_t ready_for_runtime;
        interface_regs__status__mailbox_data_avail__in_t mailbox_data_avail;
        interface_regs__status__mailbox_flow_done__in_t mailbox_flow_done;
    } interface_regs__status__in_t;

    typedef struct {
        logic [31:0] next;
    } interface_regs__cycle_count__cycle_count__in_t;

    typedef struct {
        interface_regs__cycle_count__cycle_count__in_t cycle_count;
    } interface_regs__cycle_count__in_t;

    typedef struct {
        logic [31:0] next;
    } interface_regs__fpga_version__fpga_version__in_t;

    typedef struct {
        interface_regs__fpga_version__fpga_version__in_t fpga_version;
    } interface_regs__fpga_version__in_t;

    typedef struct {
        interface_regs__generic_output_wires__in_t generic_output_wires[2];
        interface_regs__status__in_t status;
        interface_regs__cycle_count__in_t cycle_count;
        interface_regs__fpga_version__in_t fpga_version;
    } interface_regs__in_t;

    typedef struct {
        logic [7:0] next;
    } fifo_regs__log_fifo_data__next_char__in_t;

    typedef struct {
        logic next;
    } fifo_regs__log_fifo_data__char_valid__in_t;

    typedef struct {
        fifo_regs__log_fifo_data__next_char__in_t next_char;
        fifo_regs__log_fifo_data__char_valid__in_t char_valid;
    } fifo_regs__log_fifo_data__in_t;

    typedef struct {
        logic next;
    } fifo_regs__log_fifo_status__log_fifo_empty__in_t;

    typedef struct {
        logic next;
    } fifo_regs__log_fifo_status__log_fifo_full__in_t;

    typedef struct {
        fifo_regs__log_fifo_status__log_fifo_empty__in_t log_fifo_empty;
        fifo_regs__log_fifo_status__log_fifo_full__in_t log_fifo_full;
    } fifo_regs__log_fifo_status__in_t;

    typedef struct {
        logic next;
    } fifo_regs__itrng_fifo_status__itrng_fifo_empty__in_t;

    typedef struct {
        logic next;
    } fifo_regs__itrng_fifo_status__itrng_fifo_full__in_t;

    typedef struct {
        fifo_regs__itrng_fifo_status__itrng_fifo_empty__in_t itrng_fifo_empty;
        fifo_regs__itrng_fifo_status__itrng_fifo_full__in_t itrng_fifo_full;
    } fifo_regs__itrng_fifo_status__in_t;

    typedef struct {
        fifo_regs__log_fifo_data__in_t log_fifo_data;
        fifo_regs__log_fifo_status__in_t log_fifo_status;
        fifo_regs__itrng_fifo_status__in_t itrng_fifo_status;
    } fifo_regs__in_t;

    typedef struct {
        interface_regs__in_t interface_regs;
        fifo_regs__in_t fifo_regs;
    } caliptra_fpga_realtime_regs__in_t;

    typedef struct {
        logic [31:0] value;
    } interface_regs__generic_input_wires__value__out_t;

    typedef struct {
        interface_regs__generic_input_wires__value__out_t value;
    } interface_regs__generic_input_wires__out_t;

    typedef struct {
        logic [31:0] value;
    } interface_regs__generic_output_wires__value__out_t;

    typedef struct {
        interface_regs__generic_output_wires__value__out_t value;
    } interface_regs__generic_output_wires__out_t;

    typedef struct {
        logic [31:0] value;
    } interface_regs__cptra_obf_key__value__out_t;

    typedef struct {
        interface_regs__cptra_obf_key__value__out_t value;
    } interface_regs__cptra_obf_key__out_t;

    typedef struct {
        logic value;
    } interface_regs__control__cptra_pwrgood__out_t;

    typedef struct {
        logic value;
    } interface_regs__control__cptra_rst_b__out_t;

    typedef struct {
        logic value;
    } interface_regs__control__ss_debug_locked__out_t;

    typedef struct {
        logic [1:0] value;
    } interface_regs__control__ss_device_lifecycle__out_t;

    typedef struct {
        logic value;
    } interface_regs__control__scan_mode__out_t;

    typedef struct {
        logic value;
    } interface_regs__control__bootfsm_brkpoint__out_t;

    typedef struct {
        interface_regs__control__cptra_pwrgood__out_t cptra_pwrgood;
        interface_regs__control__cptra_rst_b__out_t cptra_rst_b;
        interface_regs__control__ss_debug_locked__out_t ss_debug_locked;
        interface_regs__control__ss_device_lifecycle__out_t ss_device_lifecycle;
        interface_regs__control__scan_mode__out_t scan_mode;
        interface_regs__control__bootfsm_brkpoint__out_t bootfsm_brkpoint;
    } interface_regs__control__out_t;

    typedef struct {
        logic value;
    } interface_regs__status__cptra_error_fatal__out_t;

    typedef struct {
        logic value;
    } interface_regs__status__cptra_error_non_fatal__out_t;

    typedef struct {
        logic value;
    } interface_regs__status__ready_for_fuses__out_t;

    typedef struct {
        logic value;
    } interface_regs__status__ready_for_fw_push__out_t;

    typedef struct {
        logic value;
    } interface_regs__status__ready_for_runtime__out_t;

    typedef struct {
        logic value;
    } interface_regs__status__mailbox_data_avail__out_t;

    typedef struct {
        logic value;
    } interface_regs__status__mailbox_flow_done__out_t;

    typedef struct {
        interface_regs__status__cptra_error_fatal__out_t cptra_error_fatal;
        interface_regs__status__cptra_error_non_fatal__out_t cptra_error_non_fatal;
        interface_regs__status__ready_for_fuses__out_t ready_for_fuses;
        interface_regs__status__ready_for_fw_push__out_t ready_for_fw_push;
        interface_regs__status__ready_for_runtime__out_t ready_for_runtime;
        interface_regs__status__mailbox_data_avail__out_t mailbox_data_avail;
        interface_regs__status__mailbox_flow_done__out_t mailbox_flow_done;
    } interface_regs__status__out_t;

    typedef struct {
        logic [31:0] value;
    } interface_regs__pauser__pauser__out_t;

    typedef struct {
        interface_regs__pauser__pauser__out_t pauser;
    } interface_regs__pauser__out_t;

    typedef struct {
        logic [31:0] value;
    } interface_regs__itrng_divisor__itrng_divisor__out_t;

    typedef struct {
        interface_regs__itrng_divisor__itrng_divisor__out_t itrng_divisor;
    } interface_regs__itrng_divisor__out_t;

    typedef struct {
        logic [31:0] value;
    } interface_regs__cycle_count__cycle_count__out_t;

    typedef struct {
        interface_regs__cycle_count__cycle_count__out_t cycle_count;
    } interface_regs__cycle_count__out_t;

    typedef struct {
        logic [31:0] value;
    } interface_regs__fpga_version__fpga_version__out_t;

    typedef struct {
        interface_regs__fpga_version__fpga_version__out_t fpga_version;
    } interface_regs__fpga_version__out_t;

    typedef struct {
        interface_regs__generic_input_wires__out_t generic_input_wires[2];
        interface_regs__generic_output_wires__out_t generic_output_wires[2];
        interface_regs__cptra_obf_key__out_t cptra_obf_key[8];
        interface_regs__control__out_t control;
        interface_regs__status__out_t status;
        interface_regs__pauser__out_t pauser;
        interface_regs__itrng_divisor__out_t itrng_divisor;
        interface_regs__cycle_count__out_t cycle_count;
        interface_regs__fpga_version__out_t fpga_version;
    } interface_regs__out_t;

    typedef struct {
        logic [7:0] value;
        logic rd_swacc;
    } fifo_regs__log_fifo_data__next_char__out_t;

    typedef struct {
        logic value;
    } fifo_regs__log_fifo_data__char_valid__out_t;

    typedef struct {
        fifo_regs__log_fifo_data__next_char__out_t next_char;
        fifo_regs__log_fifo_data__char_valid__out_t char_valid;
    } fifo_regs__log_fifo_data__out_t;

    typedef struct {
        logic value;
    } fifo_regs__log_fifo_status__log_fifo_empty__out_t;

    typedef struct {
        logic value;
    } fifo_regs__log_fifo_status__log_fifo_full__out_t;

    typedef struct {
        fifo_regs__log_fifo_status__log_fifo_empty__out_t log_fifo_empty;
        fifo_regs__log_fifo_status__log_fifo_full__out_t log_fifo_full;
    } fifo_regs__log_fifo_status__out_t;

    typedef struct {
        logic [31:0] value;
        logic wr_swacc;
    } fifo_regs__itrng_fifo_data__itrng_data__out_t;

    typedef struct {
        fifo_regs__itrng_fifo_data__itrng_data__out_t itrng_data;
    } fifo_regs__itrng_fifo_data__out_t;

    typedef struct {
        logic value;
    } fifo_regs__itrng_fifo_status__itrng_fifo_empty__out_t;

    typedef struct {
        logic value;
    } fifo_regs__itrng_fifo_status__itrng_fifo_full__out_t;

    typedef struct {
        logic value;
    } fifo_regs__itrng_fifo_status__itrng_fifo_reset__out_t;

    typedef struct {
        fifo_regs__itrng_fifo_status__itrng_fifo_empty__out_t itrng_fifo_empty;
        fifo_regs__itrng_fifo_status__itrng_fifo_full__out_t itrng_fifo_full;
        fifo_regs__itrng_fifo_status__itrng_fifo_reset__out_t itrng_fifo_reset;
    } fifo_regs__itrng_fifo_status__out_t;

    typedef struct {
        fifo_regs__log_fifo_data__out_t log_fifo_data;
        fifo_regs__log_fifo_status__out_t log_fifo_status;
        fifo_regs__itrng_fifo_data__out_t itrng_fifo_data;
        fifo_regs__itrng_fifo_status__out_t itrng_fifo_status;
    } fifo_regs__out_t;

    typedef struct {
        interface_regs__out_t interface_regs;
        fifo_regs__out_t fifo_regs;
    } caliptra_fpga_realtime_regs__out_t;
endpackage
