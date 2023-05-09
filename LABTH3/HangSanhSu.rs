import java.util.Date;

public class HangSanhSu extends HangHoa {
    private String nhaSanXuat;
    private Date ngayNhapKho;

    public HangSanhSu(String maHang, String tenHang, int soLuongTon, double donGia,
                      String nhaSanXuat, Date ngayNhapKho) {
        super(maHang, tenHang, soLuongTon, donGia);
        this.nhaSanXuat = nhaSanXuat;
        this.ngayNhapKho = ngayNhapKho;
    }

    public String getNhaSanXuat() {
        return nhaSanXuat;
    }

    public void setNhaSanXuat(String nhaSanXuat) {
        this.nhaSanXuat = nhaSanXuat;
    }

    public Date getNgayNhapKho() {
        return ngayNhapKho;
    }

    public void setNgayNhapKho(Date ngayNhapKho) {
        this.ngayNhapKho = ngayNhapKho;
    }

    public String danhGiaHangHoa() {
        int soLuongTon = getSoLuongTon();
        long ngayNhapKho = this.ngayNhapKho.getTime();
        long ngayHienTai = System.currentTimeMillis();
        long soNgayTrongKho = (ngayHienTai - ngayNhapKho) / (24 * 60 * 60 * 1000);

        if (soLuongTon > 50 && soNgayTrongKho > 10) {
            return "Bán chậm";
        }
        return null;
    }

    public double tinhThue() {
        return getDonGia() * getSoLuongTon() * 0.1;
    }

    @Override
    public double tinhTienVAT() {
    }

    @Override
    public String danhGiaMucDoBan() {
      
    }
}

