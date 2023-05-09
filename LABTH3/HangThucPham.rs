
import java.time.LocalDate;

class HangThucPham extends HangHoa {
    private LocalDate ngaySanXuat;
    private LocalDate ngayHetHan;
    private String nhaCungCap;

    public HangThucPham(String maHang, String tenHang, int soLuongTon, double donGia, LocalDate ngaySanXuat,
            LocalDate ngayHetHan, String nhaCungCap) {
        super(maHang, tenHang, soLuongTon, donGia);
        this.ngaySanXuat = ngaySanXuat;
        this.ngayHetHan = ngayHetHan;
        this.nhaCungCap = nhaCungCap;
    }

    public LocalDate getNgaySanXuat() {
        return ngaySanXuat;
    }

    public LocalDate getNgayHetHan() {
        return ngayHetHan;
    }

    public String getNhaCungCap() {
        return nhaCungCap;
    }

    @Override
    public double tinhTienVAT() {
        return getDonGia() * 0.05;
    }

    @Override
    public String danhGiaMucDoBan() {
        if (getSoLuongTon() == 0) {
            return "Hết hàng";
        } else if (getNgayHetHan().isBefore(LocalDate.now())) {
            return "Khó bán";
        } else {
            return "Bán được";
        }
    }
}


