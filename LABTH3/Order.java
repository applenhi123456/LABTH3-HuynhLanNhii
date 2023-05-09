package lap3_0905;

import java.time.LocalDate;
//import labth_tuan3_bai1.LABTH3.OrderDetail;

public class Order {
    private int orderID;
    private LocalDate orderDate;
    private OrderDetail[] lineItems;
    private int count;

    public Order(int orderID, LocalDate orderDate) {
        this.orderID = orderID;
        this.orderDate = orderDate;
        lineItems = new OrderDetail[10];
        count = 0;
    }

    public void addLineItem(Product p, int q) {
        lineItems[count] = new OrderDetail(p, q);
        count++;
    }

    public double calcTotalCharge() {
        double totalCharge = 0;
        for (int i = 0; i < count; i++) {
            totalCharge += lineItems[i].calcTotalPrice();
        }
        return totalCharge;
    }

    public String toString() {
        StringBuilder result = new StringBuilder();
        result.append(String.format("Mã HD: %d%n", this.orderID));
        result.append(String.format("Ngày lập hoá đơn: %s%n", this.orderDate));
        result.append(String.format("%-5s | %-20s | %-20s | %15s | %10s | %15s%n", "STT", "Mã SP", "Mô tả", "Đơn giá", "Số lượng", "Thành tiền"));

        // Duyệt qua từng sản phẩm trong đơn hàng
        for (int i = 0; i < count; i++) {
            OrderDetail item = lineItems[i];
            Product product = item.getProduct();
            int quantity = item.getQuantity();
            double price = product.getPrice();
            double total = item.calcTotalPrice();
            result.append(String.format("%-5d | %-20s | %-20s | %,15.0f | %,10d | %,15.0f%n", i+1,product.getDescription(), product.getProductID(), price, quantity, total));
        }

        // Tính tổng tiền của đơn hàng
        double totalCharge = calcTotalCharge();
        result.append(String.format("%-53s %,15.0f VND", "Tổng tiền thanh toán:", totalCharge));
        return result.toString();
    }

    public OrderDetail[] getLineItems() {
        return lineItems;
    }

    public LocalDate getOrderDate() {
        return orderDate;
    }

    public int getOrderID() {
        return orderID;
    }

    public void setOrderDate(LocalDate orderDate) {
        this.orderDate = orderDate;
    }
}