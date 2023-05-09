import java.util.ArrayList;
import java.util.Scanner;

public class QuanLyHangHoa {
    private ArrayList<HangHoa> dsHangHoa;

    public QuanLyHangHoa() {
        dsHangHoa = new ArrayList<HangHoa>();
    }

    public void themHangHoa() {
        Scanner sc = new Scanner(System.in);

        System.out.print("Loai hang hoa (1 - Thuc pham, 2 - Dien may, 3 - Sanh su): ");
        int loaiHang = sc.nextInt();

        switch (loaiHang) {
            case 1:
                ThemHangThucPham(sc);
                break;
            case 2:
                ThemHangDienMay(sc);
                break;
            case 3:
                ThemHangSanhSu(sc);
                break;
            default:
                System.out.println("Loai hang hoa khong ton tai");
                break;
        }
    }

    private void ThemHangThucPham(Scanner sc) {
        System.out.print("Nhap ma hang: ");
        String maHang = sc.next();

        System.out.print("Nhap ten hang: ");
        String tenHang = sc.next();

        System.out.print("Nhap so luong ton kho: ");
        int soLuongTonKho = sc.nextInt();

        System.out.print("Nhap don gia: ");
        double donGia = sc.nextDouble();

        System.out.print("Nhap ngay san xuat (dd/MM/yyyy): ");
        String ngaySanXuat = sc.next();

        System.out.print("Nhap ngay het han (dd/MM/yyyy): ");
        String ngayHetHan = sc.next();

        System.out.print("Nhap nha cung cap: ");
        String nhaCungCap = sc.next();

        HangThucPham hangThucPham = new HangThucPham(maHang, tenHang, soLuongTonKho, donGia, ngaySanXuat, ngayHetHan, nhaCungCap);

        if (dsHangHoa.contains(hangThucPham)) {
            System.out.println("Hang hoa da ton tai");
        } else {
            dsHangHoa.add(hangThucPham);
            System.out.println("Them thanh cong");
        }
    }

    private void ThemHangDienMay(Scanner sc) {
        System.out.print("Nhap ma hang: ");
        String maHang = sc.next();

        System.out.print("Nhap ten hang: ");
        String tenHang = sc.next();

        System.out.print("Nhap so luong ton kho: ");
        int soLuongTonKho = sc.nextInt();

        System.out.print("Nhap don gia: ");
        double donGia = sc.nextDouble();

        System.out.print("Nhap thoi gian bao hanh (thang): ");
        int thoiGianBaoHanh = sc.nextInt();

        System.out.print("Nhap cong suat (KW): ");
        double congSuat = sc.nextDouble();

        HangDienMay hangDienMay = new HangDienMay(maHang, tenHang, soLuongTonKho, donGia, thoiGianBaoHanh, congSuat);

        if (dsHangHoa.contains(hangDienMay)) {
            System.out.println("Hang hoa da ton tai");
        } else{
            dsHangHoa.add(hangDienMay);
            System.out.println("Them thanh cong");
        }
    }
private void ThemHangSanhSu(Scanner sc) {
    System.out.printf("Nhap ma hang: ");
    String maHang = sc.next();

    System.out.printf("Nhap ten hang: ");
    String tenHang = sc.next();

    System.out.printf("Nhap so luong ton kho: ");
    int soLuongTonKho = sc.nextInt();

    System.out.printf("Nhap don gia: ");
    double donGia = sc.nextDouble();

    System.out.printf("Nhap quoc gia xuat xu: ");
    String quocGiaXuatXu = sc.next();

    System.out.printf("Nhap nam san xuat: ");
    int namSanXuat = sc.nextInt();

    System.out.printf("Nhap mau sac: ");
    String mauSac = sc.next();

    HangSanhSu hangSanhSu = new HangSanhSu(maHang, tenHang, soLuongTonKho, donGia, quocGiaXuatXu, namSanXuat, mauSac);

    if (dsHangHoa.contains(hangSanhSu)) {
        System.out.println("Hang hoa da ton tai");
    } else {
        dsHangHoa.add(hangSanhSu);
        System.out.println("Them thanh cong");
    }
}

public void xoaHangHoa() {
    Scanner sc = new Scanner(System.in);

    System.out.print("Nhap ma hang hoa can xoa: ");
    String maHang = sc.next();

    boolean timThay = false;
    for (HangHoa hangHoa : dsHangHoa) {
        if (hangHoa.getMaHang().equals(maHang)) {
            dsHangHoa.remove(hangHoa);
            System.out.println("Xoa thanh cong");
            timThay = true;
            break;
        }
    }

    if (!timThay) {
        System.out.println("Khong tim thay hang hoa co ma hang " + maHang);
    }
}

public void inDanhSachHangHoa() {
    for (HangHoa hangHoa : dsHangHoa) {
        System.out.println(hangHoa);
    }
}
}


            

