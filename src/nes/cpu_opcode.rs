use super::cpu::*;
use super::system::System;
use super::interface::{SystemBus};

/// inst! macro
/// 命令のアドレッシング、フェッチ、pcのincrement、実行、クロックサイクルの返却をまとめて行います
macro_rules! inst {
    (
        $self:expr, $system:expr,
        $name:expr, pc_incr => $pc_incr:expr, cycle => $cycle:expr, 
        $addressing_closure:expr,
        $inst_closure:expr
    ) => {
        {
            if cfg!(debug_assertions) {
                println!("[before][#{}] cycle:{} pc_incr:{} pc{:04x} a:{:02x} x:{:02x} y:{:02x} sp:{:04x} p:{:08b}", $name, $cycle, $pc_incr, $self.pc, $self.a, $self.x, $self.y, $self.sp, $self.p);
            }
            // fetchしない場合(accumulate, implicit)は、pc incrementを0に設定する
            // addressはそのまま供給する
            if $pc_incr > 0 {
                let addr = $addressing_closure();
                let data = $system.read_u8(addr);
                $self.increment_pc($pc_incr);
                $inst_closure(addr, data);
            } else {
                // for implicit, accumulate
                $inst_closure(0, 0);
            }
            if cfg!(debug_assertions) {
                println!("[after][#{}] cycle:{} pc_incr:{} pc{:04x} a:{:02x} x:{:02x} y:{:02x} sp:{:04x} p:{:08b}", $name, $cycle, $pc_incr, $self.pc, $self.a, $self.x, $self.y, $self.sp, $self.p);
            }
            $cycle
        }
    };
    (
        $self:expr, $system:expr, $target_opcode:expr,
        $(
            {
                $name:expr, opcode => $opcode:expr,pc_incr => $pc_incr:expr, cycle => $cycle:expr, 
                $addressing_closure:expr,
                $inst_closure:expr
            }
        ),*
    ) => {
        match $target_opcode {
            $(
                $opcode => inst!($self, $system,
                    $name, pc_incr => $pc_incr, cycle => $cycle,
                    $addressing_closure,
                    $inst_closure
                ),
            )*
            _ => {
                panic!("invalid Operation. opcode:{:02x} pc:{:04x} a:{:02x} x:{:02x} y:{:02x} sp:{:04x} p:{:08b}", $target_opcode, $self.pc, $self.a, $self.x, $self.y, $self.sp, $self.p);
            },
        }
    }

}

/// Decode and Run
impl Cpu {
    /// 1命令実行します
    /// return: かかったclock cycle count`
    pub fn step(&mut self, system: &mut System) -> u8 {
        let opcode = system.read_u8(self.pc);
        self.increment_pc(1);
        if cfg!(debug_assertions) {
            println!("[opcode fetched] opcode:{:02x} pc:{:04x} a:{:02x} x:{:02x} y:{:02x} sp:{:04x} p:{:08b}", opcode, self.pc, self.a, self.x, self.y, self.sp, self.p);
        }
        inst!(self, system, opcode,
            /**************** ADC ****************/
            {
                "ADC imm",
                opcode => 0x69, pc_incr => 1, cycle => 2, 
                || self.addressing_immediate(system, self.pc),
                |_addr, data| self.inst_adc(data)
            },
            {
                "ADC zeropage", 
                opcode => 0x65, pc_incr => 1, cycle => 3, 
                || self.addressing_zero_page(system, self.pc),
                |_addr, data| self.inst_adc(data)
            },
            {
                "ADC zeropage x", 
                opcode => 0x75, pc_incr => 1, cycle => 4, 
                || self.addressing_zero_page_x(system, self.pc),
                |_addr, data| self.inst_adc(data)
            },
            {
                "ADC absolute", 
                opcode => 0x6d, pc_incr => 2, cycle => 4, 
                || self.addressing_absolute(system, self.pc),
                |_addr, data| self.inst_adc(data)
            },
            {
                "ADC absolute x", 
                opcode => 0x7d, pc_incr => 2, cycle => 4, 
                || self.addressing_absolute_x(system, self.pc),
                |_addr, data| self.inst_adc(data)
            },
            {
                "ADC absolute y", 
                opcode => 0x79, pc_incr => 2, cycle => 4, 
                || self.addressing_absolute_y(system, self.pc),
                |_addr, data| self.inst_adc(data)
            },
            {
                "ADC indirect x", 
                opcode => 0x61, pc_incr => 1, cycle => 6, 
                || self.addressing_indirect_x(system, self.pc),
                |_addr, data| self.inst_adc(data)
            },
            {
                "ADC indirect y", 
                opcode => 0x71, pc_incr => 1, cycle => 5, 
                || self.addressing_indirect_y(system, self.pc),
                |_addr, data| self.inst_adc(data)
            },
            /**************** AND ****************/
            {
                "AND imm",
                opcode => 0x29, pc_incr => 1, cycle => 2, 
                || self.addressing_immediate(system, self.pc),
                |_addr, data| self.inst_and(data)
            },
            {
                "AND zeropage", 
                opcode => 0x25, pc_incr => 1, cycle => 3, 
                || self.addressing_zero_page(system, self.pc),
                |_addr, data| self.inst_and(data)
            },
            {
                "AND zeropage x", 
                opcode => 0x35, pc_incr => 1, cycle => 4, 
                || self.addressing_zero_page_x(system, self.pc),
                |_addr, data| self.inst_and(data)
            },
            {
                "AND absolute", 
                opcode => 0x2d, pc_incr => 2, cycle => 4, 
                || self.addressing_absolute(system, self.pc),
                |_addr, data| self.inst_and(data)
            },
            {
                "AND absolute x", 
                opcode => 0x3d, pc_incr => 2, cycle => 4, 
                || self.addressing_absolute_x(system, self.pc),
                |_addr, data| self.inst_and(data)
            },
            {
                "AND absolute y", 
                opcode => 0x39, pc_incr => 2, cycle => 4, 
                || self.addressing_absolute_y(system, self.pc),
                |_addr, data| self.inst_and(data)
            },
            {
                "AND indirect x", 
                opcode => 0x21, pc_incr => 1, cycle => 6, 
                || self.addressing_indirect_x(system, self.pc),
                |_addr, data| self.inst_and(data)
            },
            {
                "AND indirect y", 
                opcode => 0x31, pc_incr => 1, cycle => 5, 
                || self.addressing_indirect_y(system, self.pc),
                |_addr, data| self.inst_and(data)
            },
            /**************** ASL ****************/
            {
                "ASL accumulator", 
                opcode => 0x0a, pc_incr => 0, cycle => 2, 
                || 0,
                |_addr, _data| self.inst_asl_a()
            },
            /**************** BCC ****************/
            /**************** BCS ****************/
            /**************** BEQ ****************/
            /**************** BIT ****************/
            /**************** BMI ****************/
            /**************** BNE ****************/
            /**************** BPL ****************/
            /**************** BRK ****************/
            /**************** BVC ****************/
            /**************** BVS ****************/
            /**************** CLC ****************/
            /**************** CLD ****************/
            /**************** CLI ****************/
            /**************** CLV ****************/
            /**************** CMP ****************/
            /**************** CPX ****************/
            /**************** CPY ****************/
            /**************** DEC ****************/
            /**************** DEX ****************/
            /**************** DEY ****************/
            /**************** EOR ****************/
            /**************** INC ****************/
            /**************** INX ****************/
            /**************** INY ****************/
            /**************** JMP ****************/
            /**************** JSR ****************/
            /**************** LDA ****************/
            /**************** LDX ****************/
            /**************** LDY ****************/
            /**************** LSR ****************/
            /**************** NOP ****************/
            /**************** ORA ****************/
            /**************** PHA ****************/
            /**************** PHP ****************/
            /**************** PLA ****************/
            /**************** PLP ****************/
            /**************** ROL ****************/
            /**************** ROR ****************/
            /**************** RTI ****************/
            /**************** RTS ****************/
            /**************** SBC ****************/
            /**************** SEC ****************/
            /**************** SED ****************/
            /**************** SEI ****************/
            /**************** STA ****************/
            /**************** STX ****************/
            /**************** STY ****************/
            /**************** TAX ****************/
            /**************** TAY ****************/
            /**************** TSX ****************/
            /**************** TXA ****************/
            /**************** TXS ****************/
            /**************** TYA ****************/


            {
                "Dummy", opcode => 0x00, pc_incr => 1, cycle => 1,
                || self.addressing_immediate(system, self.pc),
                |addr, data| println!("Hello macro! addr:{:04x} data:{:02x}", addr, data)
            }
        )
    }

}
