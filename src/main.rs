use std::{slice, mem::size_of, arch::x86_64::__cpuid_count};
use bitflags::bitflags;

bitflags!(
    struct FeatureInfoFlags: u64 {
        const SSE3 = 1 << 0;
        const PCLMUL = 1 << 1;
        const LZCNT = 1 << 5;
        const SSSE3 = 1 << 9;
        const FMA = 1 << 12;
        const CMPXCHG16B = 1 << 13;
        const SSE41 = 1 << 19;
        const SSE42 = 1 << 20;
        const MOVBE = 1 << 22;
        const POPCNT = 1 << 23;
        const AES = 1 << 25;
        const XSAVE = 1 << 26;
        const OSXSAVE = 1 << 27;
        const AVX = 1 << 28;
        const F16C = 1 << 29;
        const RDRND = 1 << 30;
        const HYPERVISOR = 1 << 31;
        
        const AVXVNNIINT8 = 1 << 4;
        const AVXNECONVERT = 1 << 5;
        const CMPXCHG8B = 1 << 8;
        const PREFETCHI = 1 << 14;
        const CMOX = 1 << 15;
        const MMX = 1 << 23;
        const FXSAVE = 1 << 24;
        const SSE = 1 << 25;
        const SSE2 = 1 << 26;

        const FPU = 1 << 32;
        const VME = 1 << 33;
        const DE = 1 << 34;
        const PSE = 1 << 35;
        const TSC = 1 << 36;
        const MSR = 1 << 37;
        const PAE = 1 << 38;
        const MCE = 1 << 39;
        const CX8 = 1 << 40;
        const APIC = 1 << 41;
        const SEP = 1 << 43;
        const MTRR = 1 << 44;
        const PGE = 1 << 45;
        const ACPI = 1 << 54;
        const TM = 1 << 61;
    }
);

/* Disable warning of this struct members which are never read */
#[allow(dead_code)]
struct VendorInfo {
    ebx: u32,
    ecx: u32,
    edx: u32,
}

/* Same as above */
#[allow(dead_code)]
struct FeatureInfo {
    eax: u32,
    ebx: u32,
    edx_ecx: FeatureInfoFlags,
}
pub struct CPUInfo;

impl CPUInfo {
    fn get_brand() -> String {
        unsafe {
           let cpu = __cpuid_count(0, 0);
           let vi = VendorInfo {
                ebx: cpu.ebx,
                ecx: cpu.edx,
                edx: cpu.ecx,
            };
            
            let brand = &vi as *const VendorInfo as *const u8;
            let slice = slice::from_raw_parts(brand, size_of::<VendorInfo>());
            
            format!("{}", std::str::from_utf8(slice).unwrap())
        }
    }
    
    fn has_feat(feat: FeatureInfoFlags, flag: FeatureInfoFlags) -> bool {
        if feat.contains(flag) {
            return true
        } else {
            return false
        }
    }

    fn get_cpu_feat() -> Vec<&'static str> {
        unsafe {
            let cpu = __cpuid_count(1, 0);
            let v = Some(FeatureInfo {
                eax: cpu.eax,
                ebx: cpu.ebx,
                edx_ecx: FeatureInfoFlags {
                    bits: (((cpu.edx as u64) << 32) | (cpu.ecx as u64)),
                },
            });
    
            let mut ret_vec = Vec::with_capacity(60);
            
            v.map(|feat| {
                if Self::has_feat(feat.edx_ecx, FeatureInfoFlags::ACPI) {
                    ret_vec.push("ACPI");
                }
                if Self::has_feat(feat.edx_ecx, FeatureInfoFlags::AES) {
                    ret_vec.push("AES");
                }
                if Self::has_feat(feat.edx_ecx, FeatureInfoFlags::APIC) {
                    ret_vec.push("APIC");
                }
                if Self::has_feat(feat.edx_ecx, FeatureInfoFlags::AVX) {
                    ret_vec.push("AVX");
                }
                if Self::has_feat(feat.edx_ecx, FeatureInfoFlags::AVXNECONVERT) {
                    ret_vec.push("AVXNECONVERT");
                }
                if Self::has_feat(feat.edx_ecx, FeatureInfoFlags::AVXVNNIINT8) {
                    ret_vec.push("AVXVNNIINT8");
                }
                if Self::has_feat(feat.edx_ecx, FeatureInfoFlags::CMOX) {
                    ret_vec.push("CMDX");
                }
                if Self::has_feat(feat.edx_ecx, FeatureInfoFlags::CMPXCHG16B) {
                    ret_vec.push("CMPXCHG16B");
                }
                if Self::has_feat(feat.edx_ecx, FeatureInfoFlags::CMPXCHG8B) {
                    ret_vec.push("CMPXCHG8B");
                }
                if Self::has_feat(feat.edx_ecx, FeatureInfoFlags::CX8) {
                    ret_vec.push("CX8");
                }
                if Self::has_feat(feat.edx_ecx, FeatureInfoFlags::F16C) {
                    ret_vec.push("F16C");
                }
                if Self::has_feat(feat.edx_ecx, FeatureInfoFlags::FMA) {
                    ret_vec.push("FMA");
                }
                if Self::has_feat(feat.edx_ecx, FeatureInfoFlags::FPU) {
                    ret_vec.push("FPU");
                }
                if Self::has_feat(feat.edx_ecx, FeatureInfoFlags::FXSAVE) {
                    ret_vec.push("FXSAVE");
                }
                if Self::has_feat(feat.edx_ecx, FeatureInfoFlags::HYPERVISOR) {
                    ret_vec.push("HYPERVISOR");
                }
                if Self::has_feat(feat.edx_ecx, FeatureInfoFlags::LZCNT) {
                    ret_vec.push("LZCNT");
                }
                if Self::has_feat(feat.edx_ecx, FeatureInfoFlags::MCE) {
                    ret_vec.push("MCE");
                }
                if Self::has_feat(feat.edx_ecx, FeatureInfoFlags::MMX) {
                    ret_vec.push("MMX");
                }
                if Self::has_feat(feat.edx_ecx, FeatureInfoFlags::MOVBE) {
                    ret_vec.push("MOVBE");
                }
                if Self::has_feat(feat.edx_ecx, FeatureInfoFlags::MSR) {
                    ret_vec.push("MSR");
                }
                if Self::has_feat(feat.edx_ecx, FeatureInfoFlags::MTRR) {
                    ret_vec.push("MTRR");
                }
                if Self::has_feat(feat.edx_ecx, FeatureInfoFlags::OSXSAVE) {
                    ret_vec.push("OSXSAVE");
                }
                if Self::has_feat(feat.edx_ecx, FeatureInfoFlags::PAE) {
                    ret_vec.push("PAE");
                }
                if Self::has_feat(feat.edx_ecx, FeatureInfoFlags::PCLMUL) {
                    ret_vec.push("PCLMUL");
                }
                if Self::has_feat(feat.edx_ecx, FeatureInfoFlags::PGE) {
                    ret_vec.push("PGE");
                }
                if Self::has_feat(feat.edx_ecx, FeatureInfoFlags::POPCNT) {
                    ret_vec.push("POPCNT");
                }
                if Self::has_feat(feat.edx_ecx, FeatureInfoFlags::PREFETCHI) {
                    ret_vec.push("PREFETCHI");
                }
                if Self::has_feat(feat.edx_ecx, FeatureInfoFlags::PSE) {
                    ret_vec.push("PSE");
                }
                if Self::has_feat(feat.edx_ecx, FeatureInfoFlags::RDRND) {
                    ret_vec.push("RDRND");
                }
                if Self::has_feat(feat.edx_ecx, FeatureInfoFlags::SEP) {
                    ret_vec.push("SEP");
                }
                if Self::has_feat(feat.edx_ecx, FeatureInfoFlags::SSE) {
                    ret_vec.push("SSE")
                }
                if Self::has_feat(feat.edx_ecx, FeatureInfoFlags::SSE2) {
                    ret_vec.push("SSE2")
                }
                if Self::has_feat(feat.edx_ecx, FeatureInfoFlags::SSE3) {
                    ret_vec.push("SSE3")
                }
                if Self::has_feat(feat.edx_ecx, FeatureInfoFlags::SSE41) {
                    ret_vec.push("SSE4.1")
                }
                if Self::has_feat(feat.edx_ecx, FeatureInfoFlags::SSE42) {
                    ret_vec.push("SSE4.2")
                }
                if Self::has_feat(feat.edx_ecx, FeatureInfoFlags::SSSE3) {
                    ret_vec.push("SSSE3")
                }
                if Self::has_feat(feat.edx_ecx, FeatureInfoFlags::TM) {
                    ret_vec.push("TM")
                }
                if Self::has_feat(feat.edx_ecx, FeatureInfoFlags::TSC) {
                    ret_vec.push("TSC")
                }
                if Self::has_feat(feat.edx_ecx, FeatureInfoFlags::VME) {
                    ret_vec.push("VME")
                }
                if Self::has_feat(feat.edx_ecx, FeatureInfoFlags::XSAVE) {
                    ret_vec.push("XSAVE")
                }
            });
            
            ret_vec
        }
    }
    
    fn get_stepping() -> u32 {
        unsafe {
            let cpu = __cpuid_count(1, 0);
            let step = cpu.eax & 0x000000f;
            
            step
        }
    }

    fn get_cpu_family() -> u32 {
        unsafe {
            let cpu = __cpuid_count(1, 0);
            let family = (cpu.eax & 0x0000f00) >> 8;

            family
        }
    }

    fn get_cpu_ex_family() -> u32 {
        unsafe {
            let cpu = __cpuid_count(1, 0);
            let family = (cpu.eax & 0xff00000) >> 20;

            family
        }
    }
}

fn main() {
    let brand = CPUInfo::get_brand();
    let family = CPUInfo::get_cpu_family();
    let ex_family = CPUInfo::get_cpu_ex_family();
    let stepping = CPUInfo::get_stepping();
    let feat = CPUInfo::get_cpu_feat().join(", ");

    // I like the way that how we can pass template strings rather passing them as arguments.
    println!(
"CPU Brand: {brand}
Family: {family}
Extended Family: {ex_family}
Stepping: {stepping}
Features: {feat}"
);
}
